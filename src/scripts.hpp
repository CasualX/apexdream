#pragma once

class GameContext;

class Scripts {
public:
	Scripts() = default;
	void run(GameContext& ctx);
public:
	bool bunnyhop = true;
};
