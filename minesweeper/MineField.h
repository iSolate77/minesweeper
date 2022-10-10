#include "Vei2.h"
#include "graphics.h"

class MineField {

private:
  class Tile {
  public:
    enum State { Hidden, Flagged, Revealed };

  public:
    void SpawnMine();
    bool hasMine() const;
    void Draw (const Vei2& screenPos,Graphics& gfx) const;

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
