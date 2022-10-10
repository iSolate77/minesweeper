#include "Vei2.h"

class MineField {
public:
  MineField();
  MineField(MineField &&) = default;
  MineField(const MineField &) = default;
  MineField &operator=(MineField &&) = default;
  MineField &operator=(const MineField &) = default;
  ~MineField();

private:
  class Tile {
  public:
    enum State { Hidden, Flagged, Revealed };

  public:
    void SpawnMine();
    bool hasMine() const;

  private:
    State state = State::Hidden;
    bool hasBomb = false;
  };

private:
  Tile &TileAt(const Vei2 &gridPos);

private:
  static constexpr int width = 9;
  static constexpr int height = 9;
  Tile field[width * height];

public:
  MineField(int nMines);
  void Draw(Graphics &gfx) const;
};
