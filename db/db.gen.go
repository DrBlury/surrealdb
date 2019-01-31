// Code generated by codecgen - DO NOT EDIT.

package db

import (
	"errors"
	codec1978 "github.com/ugorji/go/codec"
	"runtime"
	"strconv"
)

const (
	// ----- content types ----
	codecSelferCcUTF84526 = 1
	codecSelferCcRAW4526  = 0
	// ----- value types used ----
	codecSelferValueTypeArray4526  = 10
	codecSelferValueTypeMap4526    = 9
	codecSelferValueTypeString4526 = 6
	codecSelferValueTypeInt4526    = 2
	codecSelferValueTypeUint4526   = 3
	codecSelferValueTypeFloat4526  = 4
	codecSelferBitsize4526         = uint8(32 << (^uint(0) >> 63))
)

var (
	errCodecSelferOnlyMapOrArrayEncodeToStruct4526 = errors.New(`only encoded map or array can be decoded into a struct`)
)

type codecSelfer4526 struct{}

func init() {
	if codec1978.GenVersion != 8 {
		_, file, _, _ := runtime.Caller(0)
		panic("codecgen version mismatch: current: 8, need " + strconv.FormatInt(int64(codec1978.GenVersion), 10) + ". Re-generate file: " + file)
	}
	if false { // reference the types, but skip this branch at build/run time
	}
}

func (x *Response) CodecEncodeSelf(e *codec1978.Encoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperEncoder(e)
	_, _, _ = h, z, r
	if x == nil {
		r.EncodeNil()
	} else {
		if false {
		} else if yyxt1 := z.Extension(z.I2Rtid(x)); yyxt1 != nil {
			z.EncExtension(x, yyxt1)
		} else {
			yysep2 := !z.EncBinary()
			yy2arr2 := z.EncBasicHandle().StructToArray
			_, _ = yysep2, yy2arr2
			const yyr2 bool = false // struct tag has 'toArray'
			var yyq2 = [4]bool{     // should field at this index be written?
				x.Time != "",       // Time
				x.Status != "",     // Status
				x.Detail != "",     // Detail
				len(x.Result) != 0, // Result
			}
			_ = yyq2
			if yyr2 || yy2arr2 {
				r.WriteArrayStart(4)
			} else {
				var yynn2 int
				for _, b := range yyq2 {
					if b {
						yynn2++
					}
				}
				r.WriteMapStart(yynn2)
				yynn2 = 0
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[0] {
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Time))
					}
				} else {
					r.EncodeString(codecSelferCcUTF84526, "")
				}
			} else {
				if yyq2[0] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `time`)
					r.WriteMapElemValue()
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Time))
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[1] {
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Status))
					}
				} else {
					r.EncodeString(codecSelferCcUTF84526, "")
				}
			} else {
				if yyq2[1] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `status`)
					r.WriteMapElemValue()
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Status))
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[2] {
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Detail))
					}
				} else {
					r.EncodeString(codecSelferCcUTF84526, "")
				}
			} else {
				if yyq2[2] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `detail`)
					r.WriteMapElemValue()
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Detail))
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[3] {
					if x.Result == nil {
						r.EncodeNil()
					} else {
						if false {
						} else {
							z.F.EncSliceIntfV(x.Result, e)
						}
					}
				} else {
					r.EncodeNil()
				}
			} else {
				if yyq2[3] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `result`)
					r.WriteMapElemValue()
					if x.Result == nil {
						r.EncodeNil()
					} else {
						if false {
						} else {
							z.F.EncSliceIntfV(x.Result, e)
						}
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayEnd()
			} else {
				r.WriteMapEnd()
			}
		}
	}
}

func (x *Response) CodecDecodeSelf(d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	if false {
	} else if yyxt1 := z.Extension(z.I2Rtid(x)); yyxt1 != nil {
		z.DecExtension(x, yyxt1)
	} else {
		yyct2 := r.ContainerType()
		if yyct2 == codecSelferValueTypeMap4526 {
			yyl2 := r.ReadMapStart()
			if yyl2 == 0 {
				r.ReadMapEnd()
			} else {
				x.codecDecodeSelfFromMap(yyl2, d)
			}
		} else if yyct2 == codecSelferValueTypeArray4526 {
			yyl2 := r.ReadArrayStart()
			if yyl2 == 0 {
				r.ReadArrayEnd()
			} else {
				x.codecDecodeSelfFromArray(yyl2, d)
			}
		} else {
			panic(errCodecSelferOnlyMapOrArrayEncodeToStruct4526)
		}
	}
}

func (x *Response) codecDecodeSelfFromMap(l int, d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	var yyhl3 bool = l >= 0
	for yyj3 := 0; ; yyj3++ {
		if yyhl3 {
			if yyj3 >= l {
				break
			}
		} else {
			if r.CheckBreak() {
				break
			}
		}
		r.ReadMapElemKey()
		yys3 := z.StringView(r.DecodeStringAsBytes())
		r.ReadMapElemValue()
		switch yys3 {
		case "time":
			if r.TryDecodeAsNil() {
				x.Time = ""
			} else {
				x.Time = (string)(r.DecodeString())
			}
		case "status":
			if r.TryDecodeAsNil() {
				x.Status = ""
			} else {
				x.Status = (string)(r.DecodeString())
			}
		case "detail":
			if r.TryDecodeAsNil() {
				x.Detail = ""
			} else {
				x.Detail = (string)(r.DecodeString())
			}
		case "result":
			if r.TryDecodeAsNil() {
				x.Result = nil
			} else {
				if false {
				} else {
					z.F.DecSliceIntfX(&x.Result, d)
				}
			}
		default:
			z.DecStructFieldNotFound(-1, yys3)
		} // end switch yys3
	} // end for yyj3
	r.ReadMapEnd()
}

func (x *Response) codecDecodeSelfFromArray(l int, d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	var yyj9 int
	var yyb9 bool
	var yyhl9 bool = l >= 0
	yyj9++
	if yyhl9 {
		yyb9 = yyj9 > l
	} else {
		yyb9 = r.CheckBreak()
	}
	if yyb9 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Time = ""
	} else {
		x.Time = (string)(r.DecodeString())
	}
	yyj9++
	if yyhl9 {
		yyb9 = yyj9 > l
	} else {
		yyb9 = r.CheckBreak()
	}
	if yyb9 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Status = ""
	} else {
		x.Status = (string)(r.DecodeString())
	}
	yyj9++
	if yyhl9 {
		yyb9 = yyj9 > l
	} else {
		yyb9 = r.CheckBreak()
	}
	if yyb9 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Detail = ""
	} else {
		x.Detail = (string)(r.DecodeString())
	}
	yyj9++
	if yyhl9 {
		yyb9 = yyj9 > l
	} else {
		yyb9 = r.CheckBreak()
	}
	if yyb9 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Result = nil
	} else {
		if false {
		} else {
			z.F.DecSliceIntfX(&x.Result, d)
		}
	}
	for {
		yyj9++
		if yyhl9 {
			yyb9 = yyj9 > l
		} else {
			yyb9 = r.CheckBreak()
		}
		if yyb9 {
			break
		}
		r.ReadArrayElem()
		z.DecStructFieldNotFound(yyj9-1, "")
	}
	r.ReadArrayEnd()
}

func (x *Dispatch) CodecEncodeSelf(e *codec1978.Encoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperEncoder(e)
	_, _, _ = h, z, r
	if x == nil {
		r.EncodeNil()
	} else {
		if false {
		} else if yyxt1 := z.Extension(z.I2Rtid(x)); yyxt1 != nil {
			z.EncExtension(x, yyxt1)
		} else {
			yysep2 := !z.EncBinary()
			yy2arr2 := z.EncBasicHandle().StructToArray
			_, _ = yysep2, yy2arr2
			const yyr2 bool = false // struct tag has 'toArray'
			var yyq2 = [3]bool{     // should field at this index be written?
				x.Query != "",   // Query
				x.Action != "",  // Action
				x.Result != nil, // Result
			}
			_ = yyq2
			if yyr2 || yy2arr2 {
				r.WriteArrayStart(3)
			} else {
				var yynn2 int
				for _, b := range yyq2 {
					if b {
						yynn2++
					}
				}
				r.WriteMapStart(yynn2)
				yynn2 = 0
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[0] {
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Query))
					}
				} else {
					r.EncodeString(codecSelferCcUTF84526, "")
				}
			} else {
				if yyq2[0] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `query`)
					r.WriteMapElemValue()
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Query))
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[1] {
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Action))
					}
				} else {
					r.EncodeString(codecSelferCcUTF84526, "")
				}
			} else {
				if yyq2[1] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `action`)
					r.WriteMapElemValue()
					if false {
					} else {
						r.EncodeString(codecSelferCcUTF84526, string(x.Action))
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayElem()
				if yyq2[2] {
					if x.Result == nil {
						r.EncodeNil()
					} else {
						if false {
						} else {
							z.EncFallback(x.Result)
						}
					}
				} else {
					r.EncodeNil()
				}
			} else {
				if yyq2[2] {
					r.WriteMapElemKey()
					r.EncodeString(codecSelferCcUTF84526, `result`)
					r.WriteMapElemValue()
					if x.Result == nil {
						r.EncodeNil()
					} else {
						if false {
						} else {
							z.EncFallback(x.Result)
						}
					}
				}
			}
			if yyr2 || yy2arr2 {
				r.WriteArrayEnd()
			} else {
				r.WriteMapEnd()
			}
		}
	}
}

func (x *Dispatch) CodecDecodeSelf(d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	if false {
	} else if yyxt1 := z.Extension(z.I2Rtid(x)); yyxt1 != nil {
		z.DecExtension(x, yyxt1)
	} else {
		yyct2 := r.ContainerType()
		if yyct2 == codecSelferValueTypeMap4526 {
			yyl2 := r.ReadMapStart()
			if yyl2 == 0 {
				r.ReadMapEnd()
			} else {
				x.codecDecodeSelfFromMap(yyl2, d)
			}
		} else if yyct2 == codecSelferValueTypeArray4526 {
			yyl2 := r.ReadArrayStart()
			if yyl2 == 0 {
				r.ReadArrayEnd()
			} else {
				x.codecDecodeSelfFromArray(yyl2, d)
			}
		} else {
			panic(errCodecSelferOnlyMapOrArrayEncodeToStruct4526)
		}
	}
}

func (x *Dispatch) codecDecodeSelfFromMap(l int, d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	var yyhl3 bool = l >= 0
	for yyj3 := 0; ; yyj3++ {
		if yyhl3 {
			if yyj3 >= l {
				break
			}
		} else {
			if r.CheckBreak() {
				break
			}
		}
		r.ReadMapElemKey()
		yys3 := z.StringView(r.DecodeStringAsBytes())
		r.ReadMapElemValue()
		switch yys3 {
		case "query":
			if r.TryDecodeAsNil() {
				x.Query = ""
			} else {
				x.Query = (string)(r.DecodeString())
			}
		case "action":
			if r.TryDecodeAsNil() {
				x.Action = ""
			} else {
				x.Action = (string)(r.DecodeString())
			}
		case "result":
			if r.TryDecodeAsNil() {
				x.Result = nil
			} else {
				if false {
				} else {
					z.DecFallback(&x.Result, true)
				}
			}
		default:
			z.DecStructFieldNotFound(-1, yys3)
		} // end switch yys3
	} // end for yyj3
	r.ReadMapEnd()
}

func (x *Dispatch) codecDecodeSelfFromArray(l int, d *codec1978.Decoder) {
	var h codecSelfer4526
	z, r := codec1978.GenHelperDecoder(d)
	_, _, _ = h, z, r
	var yyj8 int
	var yyb8 bool
	var yyhl8 bool = l >= 0
	yyj8++
	if yyhl8 {
		yyb8 = yyj8 > l
	} else {
		yyb8 = r.CheckBreak()
	}
	if yyb8 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Query = ""
	} else {
		x.Query = (string)(r.DecodeString())
	}
	yyj8++
	if yyhl8 {
		yyb8 = yyj8 > l
	} else {
		yyb8 = r.CheckBreak()
	}
	if yyb8 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Action = ""
	} else {
		x.Action = (string)(r.DecodeString())
	}
	yyj8++
	if yyhl8 {
		yyb8 = yyj8 > l
	} else {
		yyb8 = r.CheckBreak()
	}
	if yyb8 {
		r.ReadArrayEnd()
		return
	}
	r.ReadArrayElem()
	if r.TryDecodeAsNil() {
		x.Result = nil
	} else {
		if false {
		} else {
			z.DecFallback(&x.Result, true)
		}
	}
	for {
		yyj8++
		if yyhl8 {
			yyb8 = yyj8 > l
		} else {
			yyb8 = r.CheckBreak()
		}
		if yyb8 {
			break
		}
		r.ReadArrayElem()
		z.DecStructFieldNotFound(yyj8-1, "")
	}
	r.ReadArrayEnd()
}
