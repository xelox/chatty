export type SequentialID = number;
export class BiSequencer {
  private _down: SequentialID = 0;
  private _up: SequentialID = 0;
  public up(): SequentialID {
    this._up += 1;
    return this._up;
  }
  public down(): SequentialID {
    this._down -= 1;
    return this._down;
  }
}
