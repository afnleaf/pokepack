# pokepaste compression
*This is an exercise of recreational programming.*

So, I play a lot of pokemon showdown (PS), I like teambuilding, I've got lots of teams in the builder. So many infact, that when I am in the "all teams" tab, stuff starts to lag. I wonder if this is due to how PS stores teams?

### The Theory

First, let's break down what a pokepaste is, here's a [team](https://pokepast.es/5f1f4494b7bd6e93) that won the latest regional tournament. EVs and all.

```
Basculegion @ Focus Sash  
Ability: Adaptability  
Level: 50  
Tera Type: Ghost  
EVs: 4 HP / 252 Atk / 252 Spe  
Adamant Nature  
- Liquidation  
- Last Respects  
- Aqua Jet  
- Protect  

Maushold-Four @ Rocky Helmet  
Ability: Friend Guard  
Level: 50  
Tera Type: Poison  
EVs: 252 HP / 4 Atk / 180 Def / 20 SpD / 52 Spe  
Jolly Nature  
- Super Fang  
- Feint  
- Follow Me  
- Protect  

Dragonite @ Loaded Dice  
Ability: Multiscale  
Level: 50  
Tera Type: Fairy  
EVs: 44 HP / 204 Atk / 4 Def / 4 SpD / 252 Spe  
Jolly Nature  
- Scale Shot  
- Tailwind  
- Haze  
- Protect  

Incineroar @ Safety Goggles  
Ability: Intimidate  
Level: 50  
Tera Type: Grass  
EVs: 196 HP / 4 Atk / 4 Def / 68 SpD / 236 Spe  
Jolly Nature  
- Flare Blitz  
- Knock Off  
- Fake Out  
- Parting Shot  

Ursaluna-Bloodmoon @ Assault Vest  
Ability: Mind's Eye  
Level: 50  
Tera Type: Fire  
EVs: 156 HP / 4 Def / 116 SpA / 100 SpD / 132 Spe  
Modest Nature  
IVs: 0 Atk  
- Blood Moon  
- Earth Power  
- Hyper Voice  
- Vacuum Wave  

Gholdengo @ Choice Specs  
Ability: Good as Gold  
Level: 50  
Tera Type: Steel  
EVs: 228 HP / 84 Def / 52 SpA / 60 SpD / 84 Spe  
Modest Nature  
- Make It Rain  
- Shadow Ball  
- Power Gem  
- Trick  
```

It's a text based format, UTF-8 or ASCII encoded? (not sure), super simle to share via websites like pokepast.es and pokebin.com. I even wrote a somewhat popular [web extension](https://chromewebstore.google.com/detail/pokepastefix/ekceaboabpgkgbpigacngnjagcdhdkmn) to fix the missing images on the currently unmaintaned pokepast.es site.

So, I was thinking, what if you could compress the informatiom in the pokepaste and make it smaller? Let's break down what information a paste contains.

```
Pokemon-Name (1427 options) -> 0-2047 -> 11 bits
Gender (M, F, genderless -) -> 2 bits
Item (537 options) -> 0-1023 -> 10 bits
Ability: (314 options) -> 0-511 -> 9 bits
32
Level: 1 -> 100 -> 0-127 -> 7 bits
Shiny: true or false -> 1 bit
8
Tera Type: (19 options) -> 0-31 -> 5 bits
EVs: 0 -> 255 x6 -> 0-255 -> 8 bits x6 = 48 bits
IVs: 0 -> 31  x6 -> 0-31 -> 5 bits x6 = 30 bits
Nature (25 options) -> 0-31 -> 5 bits
Moves: (953 options) x4 -> 0-1023 -> 10 bits x4 = 40 bits
128

we can represent everything as 3 numbers, a u32, a u8 and a u128

Add that all together and you get 168 bits
Which we can round to 168 / 8 = 21 bytes
For a team of 6, thats 126 bytes
```

If you decode the pokepaste I have included above from ASCII to bytes, you get about 1233 in total. And with UTF-8, there's variable size, each character isn't one byte. So in text encoding, each paste will be variable size, but with my custom encoding, you get can get a fixed 126 bytes per team. This looks like a guaranteed ~x10 compression to me!

My idea is to turn a single pokemon info block and compress it into a set of bytes. Everything can be represented as a number, which refers to a string value stored in an array. Thus, decoding becomes trivial o(1) lookup. You could use the same array for encoding, but that would lead to o(n) time. A hashmap is going to be faster o(1), where key is the pokemon name and value is the index which we store as bytes. This will be bigger in memory but faster in execution, cause you only need to build the "pokedex" once.

Now we had to solve the problem of bit packing, we were lucky to get something cleanly divisble by eight, 168. Too big for 128 + 64, but with some clever addition we find that we can easily represent the dex indices as a u32, u8 and u128. Why a u128 and not two u64 or four u32? Well, because of the moves, evs and ivs which are larger clumps, we can't fit it into those sizes. So, what would this look like as text?

```
2^32 = 0 -> 4,294,967,295
2^8 = 0 -> 255
2^128 = 0 -> 3.40 * 10^38 !!!
```

that's whack, maybe were better off just doing 21 u8 as hex bytes?

```
Unpacked: 050C 0002 003C 011E 0032 0000 0010 00E4 0000 0054 0034 003C 0054 000D 001F 001F 001F 001F 001F 001F 036A 0109 01AA 0121
Packed: [A1, 90, 79, 1E, 64, 87, 20, 02, A1, A1, E2, A3, 7F, FF, FF, FF, DA, 90, 96, A9, 21]
```

There thats pretty clean, can either use spaces or not.



### The Implementation

So, how are we going to get a list of all the names? I used the js package [@pkmn/dex](https://www.npmjs.com/package/@pkmn/dex) (which is part of the PS set of code), to access the data and output sorted lowercase text for each possible pokemon name, item, ability and move. This is also how I was able to confirm the count for each. Each aspect is in a different file and each element on a newline. From here we can parse and build a simple array representing each element in memory.

I am trying to think of a way to split the project up into modules, using main as just a consumer of the library. 

- we have to parse the pokepaste text format -> parser.rs
- we store that as strings in an intermediate struct -> parser.rs Pokemon, Tvs
- we need to encode to bytes -> encoder.rs, PokemonBin
- we need to decode from bytes -> decoder.rs
- we need to turn bytes back into string with the array n(1) lookup -> ???

so here is where I am stuck, encoder decoder process could be in the same module, what we are fundamentally working with are data representations:

pokepaste format <-> string pokemon struct <-> binary pokemon struct <-> packed u32, u8, u128

modules:
- dex.rs        (pokemon info)
- parser.rs     (deals with pokepaste)
- binary.rs     (deals with packing/unpacking the &[u8; 21] format)
- codec.rs      (deals with conversion between string and unpacked bin formats)

The next step is figuring out what to do with this. We have a library, sort of, but no real way to use it besides some plumber code in our main. What is the goal here? This could be a crate for people in the future, but what is the output people want? The input is clear, its the pokepaste format. The key innovation here is some sort of 10x encoded string variation of the pokepaste, but what is the chosen representation for the &[u8; 21] byte array? The module needs to stay alive somehow, otherwise there is no use in loading and building 6 vecs and hashmaps of ground truth info each time the library is called. 

Ideally this is an active pipeline that sits inside pokemon showdown and helps with minimizing team storage. It needs to be a lightweight module, a wasm module. But what is the output? A string of hex bytes? Octet? Decimal old fashioned 00001010?

Options

- raw byte array, [0-255; 21]
- hex, 2x inflation
- base64, 1.33x (url safe?)
- base122, 1.14x (this could be cool to tackle)

I have experience encoding wasm bytecode in base64, then compressing it with brotli, as a way of inlining wasm modules inside html. 

How to solve case sensitivity? This is what putting gholdengo does, defaults to 0.
```
Bulbasaur @ Choice Specs
Ability: Good as Gold
Level: 50
Shiny:
Tera Type: Steel
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Modest Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Make It Rain
- Shadow Ball
- Power Gem
- Trick
```
Check if both exact input and comparing it to_lowercase of both map/table.

To avoid having to build the dex each time the library functions are called, we make use of a global static reference to the Dex with OnceLock. Which gets parsed and built once. I wonder what implications compile time would have for this problem.

Now making the wasm modules. Can offload this to start from wasm_bindgen, this could happen when the module gets loaded. We need to expose the functions in the lib to bindgen. Have to make sure to remember how to compile with wasm-pack. Then we will move over to htmlpacker. 

### Current Output
```
Raw Bytes:
[148, 17, 90, 94, 100, 112, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 177, 181, 103, 92, 184]
[151, 113, 220, 135, 100, 48, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 41, 23, 228, 112, 184]
[28, 116, 30, 139, 100, 136, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 199, 216, 1, 208, 184]
[116, 146, 36, 25, 100, 40, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 103, 18, 196, 58, 81]
[147, 242, 25, 47, 100, 8, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 225, 91, 5, 9, 172]
[161, 145, 135, 30, 100, 128, 0, 0, 0, 0, 0, 0, 63, 255, 255, 255, 218, 144, 150, 169, 33]

Hex:
94115A5E64700000000000003FFFFFFFB1B5675CB8
9771DC8764300000000000003FFFFFFF2917E470B8
1C741E8B64880000000000003FFFFFFFC7D801D0B8
7492241964280000000000003FFFFFFF6712C43A51
93F2192F64080000000000003FFFFFFFE15B0509AC
A191871E64800000000000003FFFFFFFDA9096A921

Base64:
lBFaXmRwAAAAAAAAP////7G1Z1y4
l3Hch2QwAAAAAAAAP////ykX5HC4
HHQei2SIAAAAAAAAP////8fYAdC4
dJIkGWQoAAAAAAAAP////2cSxDpR
k/IZL2QIAAAAAAAAP////+FbBQms
oZGHHmSAAAAAAAAAP////9qQlqkh

Conversion:
Basculegion @ Focus Sash
Ability: Adaptability
Level: 50
Shiny:
Tera Type: Ghost
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Liquidation
- Last Respects
- Aqua Jet
- Protect

Maushold-Four @ Rocky Helmet
Ability: Friend Guard
Level: 50
Shiny:
Tera Type: Poison
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Super Fang
- Feint
- Follow Me
- Protect

Dragonite @ Loaded Dice
Ability: Multiscale
Level: 50
Shiny:
Tera Type: Fairy
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Scale Shot
- Tailwind
- Haze
- Protect

Incineroar @ Safety Goggles
Ability: Intimidate
Level: 50
Shiny:
Tera Type: Grass
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Flare Blitz
- Knock Off
- Fake Out
- Parting Shot

Ursaluna-Bloodmoon @ Assault Vest
Ability: Mind's Eye
Level: 50
Shiny:
Tera Type: Fire
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Blood Moon
- Earth Power
- Hyper Voice
- Vacuum Wave

Gholdengo @ Choice Specs
Ability: Good as Gold
Level: 50
Shiny:
Tera Type: Steel
EVs: 0 HP / 0 Atk / 0 Def / 0 SpA / 0 SpD / 0 Spe
Bashful Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- Make It Rain
- Shadow Ball
- Power Gem
- Trick
```



