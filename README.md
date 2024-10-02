# signals

## 16-bit vs 8-bit headers

```bash
# 16-bit headers
frames: 62 duration: 2.066666666666667s
Video saved as output.mp4
Size package: 62 payload: 16, ratio: 0.258 duration: 2.067s
Decoded: BitVec { storage: "1100111000110001", nbits: 16 }

frames: 238 duration: 7.933333333333334s
Video saved as output.mp4
Size package: 238 payload: 192, ratio: 0.807 duration: 7.933s
Decoded: Message { id: 1, content: "Hello World!" }

# 8-bit headers
frames: 46 duration: 1.5333333333333334s
Video saved as output.mp4
Decoded Package: BitVec { storage: "1100111000110001", nbits: 16 }
Size package: 46 payload: 16, ratio: 0.348 duration: 1.533s
Decoded: BitVec { storage: "1100111000110001", nbits: 16 }

frames: 222 duration: 7.4s
Video saved as output.mp4
Size package: 222 payload: 192, ratio: 0.865 duration: 7.400s
Decoded: Message { id: 1, content: "Hello World!" }
```