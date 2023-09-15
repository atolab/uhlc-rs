var searchIndex = JSON.parse('{\
"uhlc":{"doc":"A Unique Hybrid Logical Clock.","t":"RDDDSDDDDDDLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLF","n":["CSIZE","HLC","HLCBuilder","ID","MAX_SIZE","NTP64","ParseIDError","ParseNTP64Error","ParseTimestampError","SizeError","Timestamp","add","add","add","add","add","add_assign","as_secs","as_secs_f64","as_u64","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","cause","cause","cause","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","cmp","default","default","default","deserialize","deserialize","deserialize","eq","eq","eq","eq","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_str","from_str","from_str","get_delta","get_diff_duration","get_id","get_id","get_time","hash","hash","hash","into","into","into","into","into","into","into","into","into","new","new","new_timestamp","partial_cmp","partial_cmp","partial_cmp","provide","rand","serialize","serialize","serialize","size","sub","sub","sub","sub","sub","sub_assign","subsec_nanos","system_time_clock","to_duration","to_le_bytes","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","to_system_time","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","update_with_timestamp","with_clock","with_id","with_max_delta","zero_clock"],"q":[[0,"uhlc"]],"d":["The size of counter part in <code>NTP64</code> (in bits)","An Hybric Logical Clock generating <code>Timestamp</code>s","The builder of <code>HLC</code>.","An identifier for an HLC (MAX_SIZE bytes maximum). This …","The maximum size of an le-encoded <code>ID</code> in bytes: 16.","A NTP 64-bits format as specified in RFC-5909","","","","","A timestamp made of a <code>NTP64</code> and a <code>crate::HLC</code>’s unique …","","","","","","","Returns the 32-bits seconds part.","Returns this NTP64 as a f64 in seconds.","Returns this NTP64 as a u64.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Create a new <code>HLC</code> with a random u128 ID and using …","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Returns the HLC delta as <code>NTP64</code>.","","Returns the HLC <code>ID</code>.","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Constructs a new HLCBuilder for the creation of an <code>HLC</code>, …","","Generate a new <code>Timestamp</code>.","","","","","Generate a random <code>ID</code>.","","","","The size of this <code>ID</code> in bytes. I.e., the number of …","","","","","","","Returns the 32-bits fraction of second part converted to …","A physical clock relying on std::time::SystemTime::now().","Convert to a <code>Duration</code>.","This ID as bytes","","","","","","","","","","","","Convert to a <code>SystemTime</code> (making the assumption that this …","","","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","Performs the conversion. NOTE: the bytes slice is …","","","","","","","","","","","","","","","","","","","","","","","","","Update this <code>HLC</code> with a <code>Timestamp</code>.","Configure a specific physical clock for the HLC to be …","Configure a specific identifier for the HLC to be created.","Configure the maximum delta accepted by an HLC when …","A dummy clock that returns a NTP64 initialized with the …"],"i":[0,0,0,0,7,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,5,6,7,8,9,1,10,11,12,5,6,7,8,9,1,10,11,12,5,9,10,12,7,8,9,1,10,11,12,7,8,9,1,10,11,12,7,1,11,5,6,1,7,1,11,7,9,1,10,11,12,7,9,1,10,11,12,7,7,8,8,9,1,1,10,11,11,12,5,6,7,7,7,7,7,7,8,9,1,1,10,11,12,7,1,11,6,11,6,11,11,7,1,11,5,6,7,8,9,1,10,11,12,5,11,6,7,1,11,8,7,7,1,11,7,1,1,1,1,1,1,1,0,1,7,7,8,9,1,10,11,12,7,8,1,11,1,5,6,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,8,9,1,10,11,12,5,6,7,8,9,1,10,11,12,5,6,7,8,9,1,10,11,12,6,5,5,5,0],"f":[0,0,0,0,0,0,0,0,0,0,0,[[1,1]],[[1,2],1],[[1,1]],[[1,1],1],[[1,1]],[[1,2]],[1,3],[1,4],[1,2],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[5,6],0,0,0,[7,7],[8,8],[9,9],[1,1],[10,10],[11,11],[12,12],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[7,7],13],[[1,1],13],[[11,11],13],[[],5],[[],6],[[],1],[14,[[15,[7]]]],[14,[[15,[1]]]],[14,[[15,[11]]]],[[7,7],16],[[9,9],16],[[1,1],16],[[10,10],16],[[11,11],16],[[12,12],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[7,17],18],[[7,17],18],[[8,17],18],[[8,17],18],[[9,17],18],[[1,17],18],[[1,17],18],[[10,17],18],[[11,17],18],[[11,17],18],[[12,17],18],[[]],[[]],[19,7],[[]],[20,7],[21,7],[22,7],[23,7],[[]],[[]],[24,1],[[]],[[]],[[]],[[]],[25,[[15,[7]]]],[25,[[15,[1]]]],[25,[[15,[11]]]],[6,1],[[11,11],24],[6,7],[11,7],[11,1],[[7,26]],[[1,26]],[[11,26]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],5],[[1,7],11],[6,11],[[7,7],[[27,[13]]]],[[1,1],[[27,[13]]]],[[11,11],[[27,[13]]]],[28],[[],7],[[7,29],15],[[1,29],15],[[11,29],15],[7,30],[[1,1]],[[1,1],1],[[1,2],1],[[1,1]],[[1,1]],[[1,2]],[1,3],[[],1],[1,24],[7,[[32,[31]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],33],[[],33],[[],33],[[],33],[1,34],[[],15],[[],15],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[3,[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[2,[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[35,[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[36,[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[],15],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[31,[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[32,[31]]],[[15,[7]]]],[[[37,[31]]],[[15,[7]]]],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],38],[[],38],[[],38],[[],38],[[],38],[[],38],[[],38],[[],38],[[],38],[[6,11],[[15,[33]]]],[5,5],[[5,7],5],[[5,24],5],[[],1]],"c":[],"p":[[3,"NTP64"],[15,"u64"],[15,"u32"],[15,"f64"],[3,"HLCBuilder"],[3,"HLC"],[3,"ID"],[3,"SizeError"],[3,"ParseIDError"],[3,"ParseNTP64Error"],[3,"Timestamp"],[3,"ParseTimestampError"],[4,"Ordering"],[8,"Deserializer"],[4,"Result"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"NonZeroU32"],[3,"NonZeroU128"],[3,"NonZeroU8"],[3,"NonZeroU16"],[3,"NonZeroU64"],[3,"Duration"],[15,"str"],[8,"Hasher"],[4,"Option"],[3,"Demand"],[8,"Serializer"],[15,"usize"],[15,"u8"],[15,"array"],[3,"String"],[3,"SystemTime"],[15,"u128"],[15,"u16"],[15,"slice"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
