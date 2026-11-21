# samiam

**You have selected Microsoft Sam as the computer's default voice.**

Samuel Andrew Microsoft was born in '85 and has been reading errors ever since. 

[▶ Listen](https://cdn.jsdelivr.net/gh/06chaynes/samiam@main/assets/samiam.wav) or make him say something else [here](https://06chaynes.github.io/samiam/).

## npm

`npm add samiam-rs` puts Sam in a browser, types included. TypeScript needs 5.2+ and `esnext.disposable` in `lib`.

```js
import init, { Voice, phonemes } from "samiam-rs";
await init();
phonemes("Hello!");  // "HH AX L OW1 !"
new Voice().wav("I am Sam, Sam I am.");  // Uint8Array, a complete WAV
new Voice().cues("Hello!");  // Float32Array, each phoneme's start time in seconds
```

