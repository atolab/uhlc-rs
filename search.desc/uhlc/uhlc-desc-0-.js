searchState.loadedDescShard("uhlc", 0, "A Unique Hybrid Logical Clock.\nThe size of counter part in <code>NTP64</code> (in bits)\nAn Hybric Logical Clock generating <code>Timestamp</code>s\nThe builder of <code>HLC</code>.\nAn identifier for an HLC (MAX_SIZE bytes maximum). This …\nThe maximum size of an le-encoded <code>ID</code> in bytes: 16.\nA NTP 64-bits format as specified in RFC-5909\nA timestamp made of a <code>NTP64</code> and a <code>crate::HLC</code>’s unique …\nReturns the 32-bits seconds part.\nReturns this NTP64 as a f64 in seconds.\nReturns this NTP64 as a u64.\nCreate a new <code>HLC</code> with a random u128 ID and using …\nBy default formats the value as an unsigned integer in …\nFormats Timestamp as the time part followed by the ID …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the HLC delta as <code>NTP64</code>.\nReturns the HLC <code>ID</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a new HLCBuilder for the creation of an <code>HLC</code>, …\nGenerate a new <code>Timestamp</code>.\nParse a RFC3339 time representation into a NTP64.\nParse a RFC3339 time representation into a NTP64.\nGenerate a random <code>ID</code>.\nThe size of this <code>ID</code> in bytes. I.e., the number of …\nReturns the 32-bits fraction of second part converted to …\nA physical clock relying on std::time::SystemTime::now().\nConvert to a <code>Duration</code>.\nThis ID as bytes\nConvert to a RFC3339 time representation with nanoseconds …\nConvert to a RFC3339 time representation with nanoseconds …\nConvert to a <code>SystemTime</code> (making the assumption that this …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nPerforms the conversion. NOTE: the bytes slice is …\nUpdate this <code>HLC</code> with a <code>Timestamp</code>.\nConfigure a specific physical clock for the HLC to be …\nConfigure a specific identifier for the HLC to be created.\nConfigure the maximum delta accepted by an HLC when …\nA dummy clock that returns a NTP64 initialized with the …")