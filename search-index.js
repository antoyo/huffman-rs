var searchIndex = {};
searchIndex["huffman"] = {"doc":"Huffman is a library to decode huffman-encoded data.","items":[[3,"Tree","huffman","A huffman tree.",null,null],[12,"left","","",0,null],[12,"right","","",0,null],[12,"value","","",0,null],[5,"decode","","Decode the huffman-encoded `input` using the Huffman `tree`.\nThe decoding ends when `decompressed_size` is reached.",null,null],[5,"decode_with_offset","","Decode the huffman-encoded `input` using the Huffman `tree`.\nThe decoding starts at input + `offset`, where `offset` is the number bits to skip (number between\n0 and 8).\nThe decoding ends when `decompressed_size` is reached.",null,null],[11,"default","","",0,{"inputs":[],"output":{"name":"tree"}}],[11,"new","","",0,{"inputs":[],"output":{"name":"self"}}],[11,"new_with_value","","",0,{"inputs":[{"name":"u8"}],"output":{"name":"self"}}]],"paths":[[3,"Tree"]]};
searchIndex["bitreader"] = {"doc":"BitReader is a helper type to extract strings of bits from a slice of bytes.","items":[[3,"BitReader","bitreader","BitReader reads data from a byte slice at the granularity of a single bit.",null,null],[4,"BitReaderError","","Error enumeration of BitReader errors.",null,null],[13,"NotEnoughData","","Requested more bits than there are left in the byte slice at the current position.",0,null],[12,"position","bitreader::BitReaderError","",0,null],[12,"length","","",0,null],[12,"requested","","",0,null],[13,"TooManyBitsForType","bitreader","Requested more bits than the returned variable can hold, for example more than 8 bits when\nreading into a u8.",0,null],[12,"position","bitreader::BitReaderError","",0,null],[12,"requested","","",0,null],[12,"allowed","","",0,null],[6,"Result","bitreader","Result type for those BitReader operations that can fail.",null,null],[8,"ReadInto","","Helper trait to allow reading bits into a variable without explicitly mentioning its type.",null,null],[10,"read","","",1,{"inputs":[{"name":"bitreader"},{"name":"u8"}],"output":{"name":"result"}}],[11,"new","","Construct a new BitReader from a byte slice. The returned reader lives at most as long as\nthe slice given to is valid.",2,null],[11,"relative_reader","","Returns a copy of current BitReader, with the difference that its position() returns\npositions relative to the position of the original BitReader at the construction time.\nAfter construction, both readers are otherwise completely independent, except of course\nfor sharing the same source data.",2,null],[11,"read_u8","","Read at most 8 bits into a u8.",2,null],[11,"read_u16","","Read at most 16 bits into a u16.",2,null],[11,"read_u32","","Read at most 32 bits into a u32.",2,null],[11,"read_u64","","Read at most 64 bits into a u64.",2,null],[11,"read_i8","","Read at most 8 bits into a i8.\nAssumes the bits are stored in two&#39;s complement format.",2,null],[11,"read_i16","","Read at most 16 bits into a i16.\nAssumes the bits are stored in two&#39;s complement format.",2,null],[11,"read_i32","","Read at most 32 bits into a i32.\nAssumes the bits are stored in two&#39;s complement format.",2,null],[11,"read_i64","","Read at most 64 bits into a i64.\nAssumes the bits are stored in two&#39;s complement format.",2,null],[11,"skip","","Skip arbitrary number of bits. However, you can skip at most to the end of the byte slice.",2,null],[11,"position","","Returns the position of the cursor, or how many bits have been read so far.",2,null],[11,"is_aligned","","Helper to make sure the &quot;bit cursor&quot; is exactly at the beginning of a byte, or at specific\nmulti-byte alignment position.",2,null],[11,"clone","","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"fmt","","",0,null],[11,"description","","",0,null],[11,"fmt","","",0,null]],"paths":[[4,"BitReaderError"],[8,"ReadInto"],[3,"BitReader"]]};
initSearch(searchIndex);
