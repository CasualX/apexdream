#pragma once

#include <cstdint>
#include <memory>

// Initializes the time base.
void init_time();

// Returns the time in seconds since init_time was called.
double get_time();

// Sleeps the thread for the specified duration in milliseconds.
void sleep(uint32_t ms);

// Moves the mouse in the desired direction.
void mouse_move(int dx, int dy);

// Process entry.
struct ProcessEntry {
	wchar_t name[60];
	uint32_t id;
};
// Enumerates the process list.
class ProcessEnumerator {
public:
	ProcessEnumerator();
	bool next(ProcessEntry& entry);
private:
	std::unique_ptr<uint8_t[]> buffer;
	size_t buffer_len = 0;
	size_t next_offset = 0;
};

const wchar_t* const PROCESS_NAME = L"r5apex.exe";

// When debugging without EasyAntiCheat use the following PROCESS_NAME definition
// const wchar_t* const PROCESS_NAME = L"EasyAntiCheat_launcher.exe";

class GameProcess {
public:
	GameProcess(uint32_t pid);
	~GameProcess();

	// Returns if the process is still alive.
	bool heartbeat() const;

	// Gets the address of a module in the process.
	uint64_t get_module_base(const wchar_t* module_name) const;

	// Reads raw bytes from the process.
	bool read_raw(uint64_t address, void* buffer, size_t size) const;

	// Writes raw bytes to the process.
	bool write_raw(uint64_t address, const void* buffer, size_t size) const;

	// Reads a value from the process.
	template<typename T>
	bool read(uint64_t address, T& value) const;

	// Writes a value to the process.
	template<typename T>
	bool write(uint64_t address, const T& value) const;

	// Reads an array of values from the process.
	template<typename T>
	bool read_array(uint64_t address, T* array, size_t len) const;

	// Writes an array of values to the process.
	template<typename T>
	bool write_array(uint64_t address, const T* array, size_t len) const;

	// Checks if the executable matches the given metadata.
	bool check_version(uint32_t time_date_stamp, uint32_t checksum) const;

public:
	uint32_t pid = 0;
	void* process = (void*)(size_t)-1;
	uint64_t r5apex_exe = 0;
};

template<typename T>
inline bool GameProcess::read(uint64_t address, T& value) const {
	return read_raw(address, &value, sizeof(T));
}
template<typename T>
inline bool GameProcess::write(uint64_t address, const T& value) const {
	return write_raw(address, &value, sizeof(T));
}
template<typename T>
inline bool GameProcess::read_array(uint64_t address, T* array, size_t len) const {
	return read_raw(address, array, sizeof(T) * len);
}
template<typename T>
inline bool GameProcess::write_array(uint64_t address, const T* array, size_t len) const {
	return write_raw(address, array, sizeof(T) * len);
}
