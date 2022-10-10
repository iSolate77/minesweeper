#include "MineField.h"
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
