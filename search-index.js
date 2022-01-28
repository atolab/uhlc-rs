var searchIndex = JSON.parse('{\
"uhlc":{"doc":"A Unique Hybrid Logical Clock.","t":[12,17,3,3,3,18,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["0","CSIZE","HLC","HLCBuilder","ID","MAX_SIZE","NTP64","ParseIDError","ParseNTP64Error","ParseTimestampError","Timestamp","add","add","add","add_assign","as_secs","as_slice","as_u64","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","cause","cause","cause","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","cmp","default","default","default","deserialize","deserialize","deserialize","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from_str","from_str","from_str","get_diff_duration","get_id","get_time","hash","hash","hash","into","into","into","into","into","into","into","into","ne","ne","ne","ne","ne","new","new","new","new_timestamp","partial_cmp","partial_cmp","partial_cmp","serialize","serialize","serialize","size","sub","sub","sub","sub_assign","subsec_nanos","system_time_clock","to_duration","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_system_time","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","update_with_timestamp","with_clock","with_id","with_max_delta"],"q":["uhlc","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","The size of counter part in <code>NTP64</code> (in bits)","An Hybric Logical Clock generating <code>Timestamp</code>s","The builder of <code>HLC</code>.","An identifier for an HLC (MAX_SIZE bytes maximum). This …","The maximum size of an ID in bytes: 16.","A NTP 64-bits format as specified in RFC-5909","","","","A timestamp made of a <code>NTP64</code> and a <code>crate::HLC</code>’s unique …","","","","","Returns the 32-bits seconds part.","This ID as a slice","Returns this NTP64 as a u64.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Create a new <code>HLC</code> with a generated UUID and using …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Constructs a new HLCBuilder for the creation of an <code>HLC</code>, …","Create a new ID with the “<code>size</code>” first bytes of “<code>id</code>”","","Generate a new <code>Timestamp</code>.","","","","","","","The size of this ID in bytes","","","","","Returns the 32-bits fraction of second part converted to …","A physical clock relying on std::time::SystemTime::now().","Convert to a <code>Duration</code>.","","","","","","","","","","Convert to a <code>SystemTime</code> (making the assumption that this …","","","","","","","","","","","","","","","","","","","","","","","","","","Update this <code>HLC</code> with a <code>Timestamp</code>.","Configure a specific physical clock for the HLC to be …","Configure a specific identifier for the HLC to be created.","Configure the maximum delta accepted by an HLC when …"],"i":[1,0,0,0,0,2,0,0,0,0,0,1,1,1,1,1,2,1,3,4,2,5,1,6,7,8,3,4,2,5,1,6,7,8,3,5,6,8,2,5,1,6,7,8,2,5,1,6,7,8,2,1,7,3,4,1,2,1,7,2,5,1,6,7,8,2,2,5,1,1,6,7,7,8,3,4,2,2,5,1,1,6,7,8,2,1,7,7,7,7,2,1,7,3,4,2,5,1,6,7,8,5,1,6,7,8,3,2,7,4,2,1,7,2,1,7,2,1,1,1,1,1,0,1,2,5,1,6,7,8,2,1,7,1,3,4,2,2,5,1,6,7,8,3,4,2,5,1,6,7,8,3,4,2,5,1,6,7,8,4,3,3,3],"f":[null,null,null,null,null,null,null,null,null,null,null,[[]],[[["u64",15]]],[[["ntp64",3]]],[[["u64",15]]],[[],["u32",15]],[[]],[[],["u64",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["hlc",3]],null,null,null,[[],["id",3]],[[],["parseiderror",3]],[[],["ntp64",3]],[[],["parsentp64error",3]],[[],["timestamp",3]],[[],["parsetimestamperror",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["ordering",4]],[[["ntp64",3]],["ordering",4]],[[["timestamp",3]],["ordering",4]],[[]],[[]],[[],["ntp64",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["bool",15]],[[["parseiderror",3]],["bool",15]],[[["ntp64",3]],["bool",15]],[[["parsentp64error",3]],["bool",15]],[[["timestamp",3]],["bool",15]],[[["parsetimestamperror",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["uuid",3]]],[[]],[[["duration",3]],["ntp64",3]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[["str",15]],["result",4]],[[["str",15]],["result",4]],[[["timestamp",3]],["duration",3]],[[],["id",3]],[[],["ntp64",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["parseiderror",3]],["bool",15]],[[["ntp64",3]],["bool",15]],[[["parsentp64error",3]],["bool",15]],[[["timestamp",3]],["bool",15]],[[["parsetimestamperror",3]],["bool",15]],[[],["hlcbuilder",3]],[[["usize",15]],["id",3]],[[["ntp64",3],["id",3]],["timestamp",3]],[[],["timestamp",3]],[[],["option",4,[["ordering",4]]]],[[["ntp64",3]],["option",4,[["ordering",4]]]],[[["timestamp",3]],["option",4,[["ordering",4]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["usize",15]],[[]],[[["u64",15]]],[[["ntp64",3]]],[[["u64",15]]],[[],["u32",15]],[[],["ntp64",3]],[[],["duration",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["systemtime",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["timestamp",3]],["result",4,[["string",3]]]],[[],["hlcbuilder",3]],[[["id",3]],["hlcbuilder",3]],[[["duration",3]],["hlcbuilder",3]]],"p":[[3,"NTP64"],[3,"ID"],[3,"HLCBuilder"],[3,"HLC"],[3,"ParseIDError"],[3,"ParseNTP64Error"],[3,"Timestamp"],[3,"ParseTimestampError"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};