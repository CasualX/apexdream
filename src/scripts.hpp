#pragma once

class GameContext;

struct ScriptsConfig {
	bool bunnyhop = true;
	bool rapidfire = true;
	bool autoreload = true;
	bool sentinel = false;
};

class Scripts {
public:
	Scripts() = default;
	void run(GameContext& ctx);
private:
	void bunnyhop(GameContext& ctx);
	void rapidfire(GameContext& ctx);
	void autoreload(GameContext& ctx);
	void sentinel(GameContext& ctx);
public:
	ScriptsConfig config;
};
