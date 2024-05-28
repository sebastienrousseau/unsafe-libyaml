searchState.loadedDescShard("libyml", 0, "LibYML (a fork of unsafe-libyaml)\nAPI module for LibYML\nYAML API module for LibYML\nFinish a YAML stream.\nEmit a YAML document.\nEmit an event.\nFlushes the buffer of the emitter and writes the content …\nStart a YAML stream.\nParse the input stream and produce the next YAML document.\nParse the input stream and produce the next parsing event.\nScan the input stream and produce the next token.\nCreate a SCALAR event.\nLifetime marker.\nAnchor name or null.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nValue length.\nIs the tag optional for the plain style?\nIs the tag optional for any non-plain style?\nScalar style.\nTag or null.\nValue.\nCreate an ALIAS event.\nCreate a MAPPING node and attach it to the document.\nCreate a SCALAR node and attach it to the document.\nCreate a SEQUENCE node and attach it to the document.\nAdd a pair of a key and a value to a MAPPING node.\nAdd an item to a SEQUENCE node.\nDelete a YAML document and all its nodes.\nCreate the DOCUMENT-END event.\nGet a node of a YAML document.\nGet the root of a YAML document node.\nCreate a YAML document.\nCreate the DOCUMENT-START event.\nDestroy an emitter.\nInitialize an emitter.\nSet the preferred line break.\nSet if the output should be in the “canonical” format …\nSet the output encoding.\nSet the indentation increment.\nSet a generic output handler.\nSet a string output.\nSet if unescaped non-ASCII characters are allowed.\nSet the preferred line width. -1 means unlimited.\nFree any memory allocated for an event object.\nFree memory allocated by <code>yaml_malloc</code> or <code>yaml_realloc</code>.\nAllocate memory using the system’s <code>malloc</code> function.\nCreate a MAPPING-END event.\nCreate a MAPPING-START event.\nDestroy a parser.\nInitialize a parser.\nSet the source encoding.\nSet a generic input handler.\nSet a string input.\nExtend a queue by reallocating and copying the existing …\nReallocate memory using the system’s <code>realloc</code> function.\nCreate a SCALAR event.\nCreate a SEQUENCE-END event.\nCreate a SEQUENCE-START event.\nExtend a stack by reallocating and copying the existing …\nDuplicate a string using the system’s <code>strdup</code> function.\nCreate the STREAM-END event.\nCreate the STREAM-START event.\nExtend a string buffer by reallocating and copying the …\nJoin two string buffers by copying data from one to the …\nFree any memory allocated for a token object.\nRepresents the list of tag directives in a YAML document.\nRepresents the data associated with a YAML event.\nRepresents the data associated with a YAML alias event.\nRepresents the data associated with the end of a YAML …\nRepresents the data associated with the start of a YAML …\nRepresents the list of tag directives at the start of a …\nRepresents the data associated with the start of a YAML …\nRepresents the data associated with a YAML scalar event.\nRepresents the data associated with the start of a YAML …\nRepresents the data associated with the start of a YAML …\nRepresents the data associated with a YAML node.\nRepresents the data associated with a YAML mapping node.\nRepresents the data associated with a YAML scalar node.\nRepresents the data associated with a YAML sequence node.\nThe data structure for YAML tokens.\nRepresents an alias in a YAML document.\nRepresents an anchor in a YAML document.\nRepresents a scalar value in a YAML document.\nRepresents the start of a YAML data stream.\nRepresents a tag in a YAML document.\nRepresents the tag directive in a YAML document.\nRepresents the version directive in a YAML document.\nThis structure holds aliases data.\nAn alias event.\nAn alias token.\nAn anchor token.\nLet the parser choose the break type.\nLet the parser choose the encoding.\nLet the emitter choose the style.\nLet the emitter choose the style.\nLet the emitter choose the style.\nA block-end token.\nA block-entry token.\nA block-mapping-start token.\nThe block mapping style.\nA block-sequence-start token.\nThe block sequence style.\nLine break type.\nCannot compose a YAML document.\nUse CR for line breaks (Mac style).\nUse CR LN for line breaks (DOS style).\nA document-end event.\nA document-end token.\nA document-start event.\nA document-start token.\nThe document structure.\nThe double-quoted scalar style.\nExpect the first key of a block mapping.\nExpect the key of a block mapping.\nExpect a value for a simple key of a block mapping.\nExpect a value of a block mapping.\nExpect the first item of a block sequence.\nExpect an item of a block sequence.\nExpect the content of a document.\nExpect document-end.\nExpect document-start or stream-end.\nExpect nothing.\nExpect the first document-start or stream-end.\nExpect the first key of a flow mapping.\nExpect a key of a flow mapping.\nExpect a value for a simple key of a flow mapping.\nExpect a value of a flow mapping.\nExpect the first item of a flow sequence.\nExpect an item of a flow sequence.\nExpect stream-start.\nCannot emit a YAML stream.\nThe emitter states.\nThe emitter structure.\nRepresents the prefix data associated with a YAML emitter.\nThe stream encoding.\nMany bad things could happen with the parser and emitter.\nThe event structure.\nEvent types.\nA flow-entry token.\nA flow-mapping-end token.\nA flow-mapping-start token.\nThe flow mapping style.\nA flow-sequence-end token.\nA flow-sequence-start token.\nThe flow sequence style.\nThe folded scalar style.\nA key token.\nThe literal scalar style.\nUse LN for line breaks (Unix style).\nA mapping-end event.\nA mapping node.\nA mapping-start event.\nMapping styles.\nThe pointer position.\nCannot allocate or reallocate a block of memory.\nNo error is produced.\nAn empty event.\nAn empty node.\nAn empty token.\nRepresents an element of a YAML sequence node.\nAn element of a mapping node.\nThe node structure.\nNode types.\nExpect the first key of a block mapping.\nExpect a block mapping key.\nExpect a block mapping value.\nExpect a block node or indentless sequence.\nExpect a block node.\nExpect an entry of a block sequence.\nExpect the first entry of a block sequence.\nExpect the content of a document.\nExpect document-end.\nExpect document-start.\nExpect nothing.\nExpect an empty value of a flow mapping.\nExpect the first key of a flow mapping.\nExpect a key of a flow mapping.\nExpect a value of a flow mapping.\nExpect a flow node.\nExpect the and of an ordered mapping entry.\nExpect a key of an ordered mapping.\nExpect a value of an ordered mapping.\nExpect an entry of a flow sequence.\nExpect the first entry of a flow sequence.\nExpect the beginning of an implicit document.\nExpect an entry of an indentless sequence.\nExpect stream-start.\nCannot parse the input stream.\nThe states of the parser.\nThe parser structure.\nRepresents the prefix data associated with a YAML parser.\nThe plain scalar style.\nThe prototype of a read handler.\nCannot read or decode the input stream.\nA scalar event.\nA scalar node.\nScalar styles.\nA scalar token.\nCannot scan the input stream.\nA sequence-end event.\nA sequence node.\nA sequence-start event.\nSequence styles.\nThis structure holds information about a potential simple …\nThe single-quoted scalar style.\nThe beginning of the stack.\nA stream-end event.\nA stream-end token.\nA stream-start event.\nA stream-start token.\nThe tag directive data.\nA tag-directive token.\nA tag token.\nThe token structure.\nThe token types.\nThe UTF-16-BE encoding with BOM.\nThe UTF-16-LE encoding with BOM.\nThe default UTF-8 encoding.\nA value token.\nThe version directive data.\nA version-directive token.\nThe prototype of a write handler.\nCannot write to the output stream.\nThe alias (for YamlAliasToken).\nThe alias parameters (for YamlAliasEvent).\nThe anchor (for YamlAnchorToken).\nThe anchor.\nThe anchor.\nThe anchor.\nThe anchor.\nThe anchor.\nThe position column.\nThe error context.\nThe error context.\nThe context position.\nThe context position.\nThe token data.\nThe event data.\nThe node data.\nThe document end parameters (for YamlDocumentEndEvent).\nThe document parameters (for YamlDocumentStartEvent).\nThe stream encoding.\nThe document encoding.\nThe end of the tag directives list.\nThe end of the tag directives list.\nThe end of the stack.\nIs the document end indicator implicit?\nThe end of the token.\nThe end of the event.\nThe end of the node.\nThe end of the document.\nError type.\nThe error type.\nError type.\nThe error type.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe tag handle.\nThe tag handle.\nThe tag handle.\nIs the document indicator implicit?\nIs the document end indicator implicit?\nIs the tag optional?\nIs the tag optional?\nThe position index.\nThe node id.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe stack of sequence items.\nThe key of the element.\nThe length of the scalar value.\nThe length of the scalar value.\nThe length of the scalar value.\nThe position line.\nThe major version number.\nThe major version number.\nThe mapping parameters (for YamlMappingNode).\nThe mapping parameters (for YamlMappingStartEvent).\nThe position mark.\nThe anchor mark.\nThe minor version number.\nThe minor version number.\nThe document nodes.\nThe stack of mapping pairs (key, value).\nIs the tag optional for the plain style?\nIs a simple key possible?\nThe tag prefix.\nThe tag prefix.\nError description.\nThe error description.\nError description.\nThe error description.\nThe problem position.\nThe problem position.\nThe byte about which the problem occurred.\nThe byte about which the problem occurred.\nThe problematic value (-1 is none).\nThe problematic value (-1 is none).\nIs the tag optional for any non-plain style?\nIs a simple key required?\nThe scalar value (for YamlScalarToken).\nThe scalar parameters (for YamlScalarEvent).\nThe scalar parameters (for YamlScalarNode).\nThe sequence parameters (for YamlSequenceNode).\nThe sequence parameters (for YamlSequenceStartEvent).\nThe beginning of the tag directives list.\nThe beginning of the tag directives list.\nThe beginning of the stack.\nIs the document start indicator implicit?\nThe beginning of the token.\nThe beginning of the event.\nThe beginning of the node.\nThe beginning of the document.\nThe stream start (for YamlStreamStartToken).\nThe stream parameters (for YamlStreamStartEvent).\nThe scalar style.\nThe scalar style.\nThe sequence style.\nThe mapping style.\nThe scalar style.\nThe sequence style.\nThe mapping style.\nThe tag suffix.\nThe tag (for YamlTagToken).\nThe tag.\nThe tag.\nThe tag.\nThe node tag.\nThe tag directive (for YamlTagDirectiveToken).\nThe list of tag directives.\nThe list of tag directives.\nThe number of the token.\nThe top of the stack.\nThe token type.\nThe event type.\nThe node type.\nThe alias value.\nThe anchor value.\nThe scalar value.\nThe scalar value.\nThe scalar value.\nThe value of the element.\nThe version directive (for YamlVersionDirectiveToken).\nThe version directive.\nThe version directive.")