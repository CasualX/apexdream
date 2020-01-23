#pragma once

class GameContext;

struct ScriptsConfig {
	bool bunnyhop = true;
};

class Scripts {
public:
	Scripts() = default;
	void run(GameContext& ctx);
public:
	ScriptsConfig config;
};
