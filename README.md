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

- hex, 2x inflation
- base64, 1.33x
- base122, 1.14x (this could be cool to tackle)

I have experience encoding wasm bytecode in base64, then compressing it with brotli, as a way of inlining wasm modules inside html. 

### Current Output
```
basculegion @ focus sash
Ability: adaptability
Level: 50
Shiny: 
Tera Type: ghost
EVs: 4 HP / 252 Atk /  Def /  SpA /  SpD / 252 Spe
adamant Nature
IVs:  HP /  Atk /  Def /  SpA /  SpD /  Spe
Moves:
- liquidation
- last respects
- aqua jet
- protect

maushold-four @ rocky helmet
Ability: friend guard
Level: 50
Shiny: 
Tera Type: poison
EVs: 252 HP / 4 Atk / 180 Def /  SpA / 20 SpD / 52 Spe
jolly Nature
IVs:  HP /  Atk /  Def /  SpA /  SpD /  Spe
Moves:
- super fang
- feint
- follow me
- protect

dragonite @ loaded dice
Ability: multiscale
Level: 50
Shiny: 
Tera Type: fairy
EVs: 44 HP / 204 Atk / 4 Def /  SpA / 4 SpD / 252 Spe
jolly Nature
IVs:  HP /  Atk /  Def /  SpA /  SpD /  Spe
Moves:
- scale shot
- tailwind
- haze
- protect

incineroar @ safety goggles
Ability: intimidate
Level: 50
Shiny: 
Tera Type: grass
EVs: 196 HP / 4 Atk / 4 Def /  SpA / 68 SpD / 236 Spe
jolly Nature
IVs:  HP /  Atk /  Def /  SpA /  SpD /  Spe
Moves:
- flare blitz
- knock off
- fake out
- parting shot

ursaluna-bloodmoon @ assault vest
Ability: mind's eye
Level: 50
Shiny: 
Tera Type: fire
EVs: 156 HP /  Atk / 4 Def / 116 SpA / 100 SpD / 132 Spe
modest Nature
IVs:  HP / 0 Atk /  Def /  SpA /  SpD /  Spe
Moves:
- blood moon
- earth power
- hyper voice
- vacuum wave

gholdengo @ choice specs
Ability: good as gold
Level: 50
Shiny: 
Tera Type: steel
EVs: 228 HP /  Atk / 84 Def / 52 SpA / 60 SpD / 84 Spe
modest Nature
IVs:  HP /  Atk /  Def /  SpA /  SpD /  Spe
Moves:
- make it rain
- shadow ball
- power gem
- trick

Packed: [94, 11, 08, 5E, 64, 70, 27, E0, 00, 00, 07, E1, 7F, FF, FF, FF, B1, B5, 67, 5C, B8]
Unpacked: 04A0 0002 0084 005E 0032 0000 000E 0004 00FC 0000 0000 0000 00FC 0005 001F 001F 001F 001F 001F 001F 02C6 0356 01D7 00B8
String:
basculegion @ focus sash
Ability: adaptability
Level: 50
Shiny: 
Tera Type: ghost
EVs: 4 HP / 252 Atk / 0 Def / 0 SpA / 0 SpD / 252 Spe
adamant Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- liquidation
- last respects
- aqua jet
- protect

Packed: [97, 72, 8E, 87, 64, 37, E0, 25, A0, 00, A1, A5, BF, FF, FF, FF, 29, 17, E4, 70, B8]
Unpacked: 04BB 0002 0147 0087 0032 0000 0006 00FC 0004 00B4 0000 0014 0034 0016 001F 001F 001F 001F 001F 001F 00A4 017E 011C 00B8
String:
maushold-four @ rocky helmet
Ability: friend guard
Level: 50
Shiny: 
Tera Type: poison
EVs: 252 HP / 4 Atk / 180 Def / 0 SpA / 20 SpD / 52 Spe
jolly Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- super fang
- feint
- follow me
- protect

Packed: [1C, 71, A2, 8B, 64, 89, 66, 60, 20, 00, 27, E5, BF, FF, FF, FF, C7, D8, 01, D0, B8]
Unpacked: 00E3 0002 00D1 008B 0032 0000 0011 002C 00CC 0004 0000 0004 00FC 0016 001F 001F 001F 001F 001F 001F 031F 0180 0074 00B8
String:
dragonite @ loaded dice
Ability: multiscale
Level: 50
Shiny: 
Tera Type: fairy
EVs: 44 HP / 204 Atk / 4 Def / 0 SpA / 4 SpD / 252 Spe
jolly Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- scale shot
- tailwind
- haze
- protect

Packed: [74, 92, A4, 19, 64, 2E, 20, 20, 20, 02, 27, 65, BF, FF, FF, FF, 67, 12, C4, 3A, 51]
Unpacked: 03A4 0002 0152 0019 0032 0000 0005 00C4 0004 0004 0000 0044 00EC 0016 001F 001F 001F 001F 001F 001F 019C 012C 010E 0251
String:
incineroar @ safety goggles
Ability: intimidate
Level: 50
Shiny: 
Tera Type: grass
EVs: 196 HP / 4 Atk / 4 Def / 0 SpA / 68 SpD / 236 Spe
jolly Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- flare blitz
- knock off
- fake out
- parting shot

Packed: [93, F0, 25, 2F, 64, 0C, E0, 00, 23, A3, 24, 23, 7E, 0F, FF, FF, E1, 5B, 05, 09, AC]
Unpacked: 049F 0002 0012 012F 0032 0000 0001 009C 0000 0004 0074 0064 0084 000D 001F 0000 001F 001F 001F 001F 0385 01B0 0142 01AC
String:
ursaluna-bloodmoon @ assault vest
Ability: mind's eye
Level: 50
Shiny: 
Tera Type: fire
EVs: 156 HP / 0 Atk / 4 Def / 116 SpA / 100 SpD / 132 Spe
modest Nature
IVs: 31 HP / 0 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- blood moon
- earth power
- hyper voice
- vacuum wave

Packed: [A1, 90, 79, 1E, 64, 87, 20, 02, A1, A1, E2, A3, 7F, FF, FF, FF, DA, 90, 96, A9, 21]
Unpacked: 050C 0002 003C 011E 0032 0000 0010 00E4 0000 0054 0034 003C 0054 000D 001F 001F 001F 001F 001F 001F 036A 0109 01AA 0121
String:
gholdengo @ choice specs
Ability: good as gold
Level: 50
Shiny: 
Tera Type: steel
EVs: 228 HP / 0 Atk / 84 Def / 52 SpA / 60 SpD / 84 Spe
modest Nature
IVs: 31 HP / 31 Atk / 31 Def / 31 SpA / 31 SpD / 31 Spe
Moves:
- make it rain
- shadow ball
- power gem
- trick
```



