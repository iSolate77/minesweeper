#include "MineField.h"
#include "SpriteCodex.h"
#include "Vei2.h"
#include "graphics.h"
#include <assert.h>
#include <random>

void MineField::Tile::SpawnMine() {
  assert(!hasBomb);
  hasBomb = true;
}

bool MineField::Tile::hasMine() const { return hasBomb; }

MineField::MineField(int nMines) {
  assert(nMines > 0 && nMines < width * height);
  std::random_device rd;
  std::mt19937 rng(rd());
  std::uniform_int_distribution<int> xDist(0, width - 1);
  std::uniform_int_distribution<int> yDist(0, height - 1);

  for (int nSpawned = 0; nSpawned < nMines; ++nSpawned) {
    Vei2 spawnPos;
    do {
      spawnPos = {xDist(rng), yDist(rng)};
    } while (TileAt(spawnPos).hasMine());

    TileAt(spawnPos).SpawnMine();
  }
}

MineField::Tile &MineField::TileAt(const Vei2 &gridPos) {
  return field[gridPos.y * width + gridPos.x];
}

void MineField::Draw(Graphics &gfx) const {
  for (Vei2 gridPos = {0, 0}; gridPos.y < height; gridPos.y++) {
    for (; gridPos.x < width; gridPos.x++) {
      TileAt(gridPos);
    }
  }
}

void MineField::Tile::Draw(const Vei2 &screenPos, Graphics &gfx) const {
  switch (state) {
  case State::Hidden:
    SpriteCodex::DrawTileButton(screenPos, gfx);
    break;
  case State::Flagged:
    SpriteCodex::DrawTileButton(screenPos, gfx);
    SpriteCodex::DrawTileFlag(screenPos, gfx);
    break;
  case State::Revealed:
    if (!hasBomb) {
      SpriteCodex::DrawTile0(screenPos, gfx)
    } else {
      SpriteCodex::DrawTileBomb(screenPos, gfx)
    }
    break;
  }
}
