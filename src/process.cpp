#include "process.hpp"

#include <memory>
#include <cinttypes>

#include <Windows.h>
#include <winternl.h>

static LARGE_INTEGER TIME_START;
static LARGE_INTEGER TIME_FREQ;

void init_time() {
	QueryPerformanceCounter(&TIME_START);
	QueryPerformanceFrequency(&TIME_FREQ);
}
double get_time() {
	LARGE_INTEGER time;
	QueryPerformanceCounter(&time);
	return static_cast<double>(time.QuadPart - TIME_START.QuadPart) / static_cast<double>(TIME_FREQ.QuadPart);
}

void sleep(uint32_t ms) {
	SleepEx(ms, TRUE);
}

void mouse_move(int dx, int dy) {
	INPUT input{INPUT_MOUSE};
	input.mi = MOUSEINPUT {dx, dy, 0, MOUSEEVENTF_MOVE, 0, 0};
	SendInput(1, &input, sizeof(INPUT));
}

//----------------------------------------------------------------

ProcessEnumerator::ProcessEnumerator() {
	using NtQuerySystemInformationFn = NTSTATUS WINAPI(IN SYSTEM_INFORMATION_CLASS, OUT PVOID, IN ULONG, OUT PULONG);
	const auto NtQuerySystemInformation = reinterpret_cast<NtQuerySystemInformationFn*>(GetProcAddress(GetModuleHandleW(L"ntdll.dll"), "NtQuerySystemInformation"));
	ULONG return_length;
	while (NtQuerySystemInformation(SystemProcessInformation, buffer.get(), static_cast<ULONG>(buffer_len), &return_length) < 0) {
		buffer = std::unique_ptr<uint8_t[]>(new uint8_t[return_length]);
		buffer_len = return_length;
	}
}
bool ProcessEnumerator::next(ProcessEntry& entry) {
	if (next_offset >= buffer_len) {
		return false;
	}
	const auto pi = reinterpret_cast<const SYSTEM_PROCESS_INFORMATION*>(buffer.get() + next_offset);
	next_offset += pi->NextEntryOffset != 0 ? pi->NextEntryOffset : buffer_len - next_offset;

	entry.id = static_cast<uint32_t>((size_t)pi->UniqueProcessId);
	memset(&entry.name, 0, sizeof(entry.name));
	memcpy(&entry.name, pi->ImageName.Buffer, min(pi->ImageName.Length, sizeof(entry.name) - 1));

	return true;
}

//----------------------------------------------------------------
// EAC BYPASS GOES HERE

bool read_process_memory(uint32_t pid, uint64_t address, void* buffer, size_t size) {
	return false;
}
bool write_process_memory(uint32_t pid, uint64_t address, const void* buffer, size_t size) {
	return false;
}
bool virtual_query_ex(uint32_t pid, uint64_t address, MEMORY_BASIC_INFORMATION& mbi) {
	return false;
}
const wchar_t* get_mapped_file_name(uint32_t pid, uint64_t address, void* buffer, size_t size) {
	return nullptr;
}

//----------------------------------------------------------------

GameProcess::GameProcess(uint32_t pid) : pid(pid) {
	printf("apex(%u) Attached!\n", pid);
	r5apex_exe = get_module_base(L"r5apex.exe");
	printf("apex(%u) 0x%" PRIx64 " r5apex.exe\n", pid, r5apex_exe);
}
GameProcess::~GameProcess() {
	printf("apex(%u) Detached!\n", pid);
}

bool GameProcess::heartbeat() const {
	uint16_t dummy;
	return read(r5apex_exe, dummy);
}
uint64_t GameProcess::get_module_base(const wchar_t* module_name) const {
	MEMORY_BASIC_INFORMATION mbi;
	wchar_t buffer[MAX_PATH];
	for (uint64_t address = 0; virtual_query_ex(pid, address, mbi); address += mbi.RegionSize) {
		if (mbi.State == MEM_COMMIT && mbi.Type == MEM_IMAGE) {
			if (const auto path = get_mapped_file_name(pid, address, buffer, sizeof(buffer))) {
				size_t offset = 0;
				for (size_t i = 0; path[i] != L'\0'; i += 1) {
					if (path[i] == '\\') {
						offset = i + 1;
					}
				}
				const auto file_name = path + offset;
				if (!wcscmp(file_name, module_name)) {
					return address;
				}
			}
		}
	}
	return 0;
}
bool GameProcess::read_raw(uint64_t address, void* buffer, size_t size) const {
	return read_process_memory(pid, address, buffer, size);
}
bool GameProcess::write_raw(uint64_t address, const void* buffer, size_t size) const {
	return write_process_memory(pid, address, buffer, size);
}
bool GameProcess::check_version(uint32_t time_date_stamp, uint32_t checksum) const {
	IMAGE_DOS_HEADER dos_header;
	IMAGE_NT_HEADERS64 nt_headers;
	if (!(read(r5apex_exe, dos_header) && read(r5apex_exe + dos_header.e_lfanew, nt_headers))) {
		return false;
	}

	// If TimeDateStamp and CheckSum match then our offsets are probably up-to-date
	if (nt_headers.FileHeader.TimeDateStamp == time_date_stamp && nt_headers.OptionalHeader.CheckSum == checksum) {
		return true;
	}

	// Wait a minute to give the game a chance to decrypt itself
	sleep(1000 * 60);

	// Dump the game binary from memory
	const size_t target_len = nt_headers.OptionalHeader.SizeOfImage;
	auto target = std::unique_ptr<uint8_t[]>(new uint8_t[target_len]);
	if (read_array(r5apex_exe, target.get(), target_len)) {
		// Fixup section headers...
		auto pnt_headers = reinterpret_cast<PIMAGE_NT_HEADERS64>(target.get() + dos_header.e_lfanew);
		auto section_headers = reinterpret_cast<PIMAGE_SECTION_HEADER>(
			target.get() +
			dos_header.e_lfanew +
			FIELD_OFFSET(IMAGE_NT_HEADERS, OptionalHeader) +
			nt_headers.FileHeader.SizeOfOptionalHeader);
		for (size_t i = 0; i < nt_headers.FileHeader.NumberOfSections; i += 1) {
			auto& section = section_headers[i];
			// Rewrite the file offsets to the virtual addresses
			section.PointerToRawData = section.VirtualAddress;
			section.SizeOfRawData = section.Misc.VirtualSize;
			// Rewrite the base relocations to the ".reloc" section
			if (!memcmp(section.Name, ".reloc\0\0", 8)) {
				pnt_headers->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC] = {
					section.VirtualAddress,
					section.Misc.VirtualSize,
				};
			}
		}

		const auto dump_file = CreateFileW(L"r5apex.dump", GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_COMPRESSED, NULL);
		if (dump_file != INVALID_HANDLE_VALUE) {
			if (!WriteFile(dump_file, target.get(), static_cast<DWORD>(target_len), NULL, NULL)) {
				printf("apex(%u) Error writing r5apex.dump: %u\n", pid, GetLastError());
			}
			CloseHandle(dump_file);
		}
		else {
			printf("apex(%u) Error writing r5apex.dump: %u\n", pid, GetLastError());
		}
	}

	return false;
}
