#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mo0: MO,
    mo1: MO,
    mo2: MO,
    mo3: MO,
    mo4: MO,
    mo5: MO,
    mo6: MO,
    mo7: MO,
    mo8: MO,
    mo9: MO,
    mo10: MO,
    mo11: MO,
    mo12: MO,
    mo13: MO,
    mo14: MO,
    mo15: MO,
    mo16: MO,
    mo17: MO,
    mo18: MO,
    mo19: MO,
    mo20: MO,
    mo21: MO,
    mo22: MO,
    mo23: MO,
    mo24: MO,
    mo25: MO,
    mo26: MO,
    mo27: MO,
    mo28: MO,
    mo29: MO,
    mo30: MO,
    mo31: MO,
    mo32: MO,
    mo33: MO,
    mo34: MO,
    mo35: MO,
    mo36: MO,
    mo37: MO,
    mo38: MO,
    mo39: MO,
    mo40: MO,
    mo41: MO,
    mo42: MO,
    mo43: MO,
    mo44: MO,
    mo45: MO,
    mo46: MO,
    mo47: MO,
    mo48: MO,
    mo49: MO,
    mo50: MO,
    mo51: MO,
    mo52: MO,
    mo53: MO,
    mo54: MO,
    mo55: MO,
    mo56: MO,
    mo57: MO,
    mo58: MO,
    mo59: MO,
    mo60: MO,
    mo61: MO,
    mo62: MO,
    mo63: MO,
    mo64: MO,
    mo65: MO,
    mo66: MO,
    mo67: MO,
    mo68: MO,
    mo69: MO,
    mo70: MO,
    mo71: MO,
    mo72: MO,
    mo73: MO,
    mo74: MO,
    mo75: MO,
    mo76: MO,
    mo77: MO,
    mo78: MO,
    mo79: MO,
    mo80: MO,
    mo81: MO,
    mo82: MO,
    mo83: MO,
    mo84: MO,
    mo85: MO,
    mo86: MO,
    mo87: MO,
    mo88: MO,
    mo89: MO,
    mo90: MO,
    mo91: MO,
    mo92: MO,
    mo93: MO,
    mo94: MO,
    mo95: MO,
    mo96: MO,
    mo97: MO,
    mo98: MO,
    mo99: MO,
    mo100: MO,
    mo101: MO,
    mo102: MO,
    mo103: MO,
    mo104: MO,
    mo105: MO,
    mo106: MO,
    mo107: MO,
    mo108: MO,
    mo109: MO,
    mo110: MO,
    mo111: MO,
    mo112: MO,
    mo113: MO,
    mo114: MO,
    mo115: MO,
    mo116: MO,
    mo117: MO,
    mo118: MO,
    mo119: MO,
    mo120: MO,
    mo121: MO,
    mo122: MO,
    mo123: MO,
    mo124: MO,
    mo125: MO,
    mo126: MO,
    mo127: MO,
    mo128: MO,
    mo129: MO,
    mo130: MO,
    mo131: MO,
    mo132: MO,
    mo133: MO,
    mo134: MO,
    mo135: MO,
    mo136: MO,
    mo137: MO,
    mo138: MO,
    mo139: MO,
    mo140: MO,
    mo141: MO,
    mo142: MO,
    mo143: MO,
    mo144: MO,
    mo145: MO,
    mo146: MO,
    mo147: MO,
    mo148: MO,
    mo149: MO,
    mo150: MO,
    mo151: MO,
    mo152: MO,
    mo153: MO,
    mo154: MO,
    mo155: MO,
    mo156: MO,
    mo157: MO,
    mo158: MO,
    mo159: MO,
    mo160: MO,
    mo161: MO,
    mo162: MO,
    mo163: MO,
    mo164: MO,
    mo165: MO,
    mo166: MO,
    mo167: MO,
    mo168: MO,
    mo169: MO,
    mo170: MO,
    mo171: MO,
    mo172: MO,
    mo173: MO,
    mo174: MO,
    mo175: MO,
    mo176: MO,
    mo177: MO,
    mo178: MO,
    mo179: MO,
    mo180: MO,
    mo181: MO,
    mo182: MO,
    mo183: MO,
    mo184: MO,
    mo185: MO,
    mo186: MO,
    mo187: MO,
    mo188: MO,
    mo189: MO,
    mo190: MO,
    mo191: MO,
    mo192: MO,
    mo193: MO,
    mo194: MO,
    mo195: MO,
    mo196: MO,
    mo197: MO,
    mo198: MO,
    mo199: MO,
    mo200: MO,
    mo201: MO,
    mo202: MO,
    mo203: MO,
    mo204: MO,
    mo205: MO,
    mo206: MO,
    mo207: MO,
    mo208: MO,
    mo209: MO,
    mo210: MO,
    mo211: MO,
    mo212: MO,
    mo213: MO,
    mo214: MO,
    mo215: MO,
    mo216: MO,
    mo217: MO,
    mo218: MO,
    mo219: MO,
    mo220: MO,
    mo221: MO,
    mo222: MO,
    mo223: MO,
    mo224: MO,
    mo225: MO,
    mo226: MO,
    mo227: MO,
    mo228: MO,
    mo229: MO,
    mo230: MO,
    mo231: MO,
    mo232: MO,
    mo233: MO,
    mo234: MO,
    mo235: MO,
    mo236: MO,
    mo237: MO,
    mo238: MO,
    mo239: MO,
    mo240: MO,
    mo241: MO,
    mo242: MO,
    mo243: MO,
    mo244: MO,
    mo245: MO,
    mo246: MO,
    mo247: MO,
    mo248: MO,
    mo249: MO,
    mo250: MO,
    mo251: MO,
    mo252: MO,
    mo253: MO,
    mo254: MO,
    mo255: MO,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo0(&self) -> &MO {
        &self.mo0
    }
    #[doc = "0x20..0x40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo1(&self) -> &MO {
        &self.mo1
    }
    #[doc = "0x40..0x60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo2(&self) -> &MO {
        &self.mo2
    }
    #[doc = "0x60..0x80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo3(&self) -> &MO {
        &self.mo3
    }
    #[doc = "0x80..0xa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo4(&self) -> &MO {
        &self.mo4
    }
    #[doc = "0xa0..0xc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo5(&self) -> &MO {
        &self.mo5
    }
    #[doc = "0xc0..0xe0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo6(&self) -> &MO {
        &self.mo6
    }
    #[doc = "0xe0..0x100 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo7(&self) -> &MO {
        &self.mo7
    }
    #[doc = "0x100..0x120 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo8(&self) -> &MO {
        &self.mo8
    }
    #[doc = "0x120..0x140 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo9(&self) -> &MO {
        &self.mo9
    }
    #[doc = "0x140..0x160 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo10(&self) -> &MO {
        &self.mo10
    }
    #[doc = "0x160..0x180 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo11(&self) -> &MO {
        &self.mo11
    }
    #[doc = "0x180..0x1a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo12(&self) -> &MO {
        &self.mo12
    }
    #[doc = "0x1a0..0x1c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo13(&self) -> &MO {
        &self.mo13
    }
    #[doc = "0x1c0..0x1e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo14(&self) -> &MO {
        &self.mo14
    }
    #[doc = "0x1e0..0x200 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo15(&self) -> &MO {
        &self.mo15
    }
    #[doc = "0x200..0x220 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo16(&self) -> &MO {
        &self.mo16
    }
    #[doc = "0x220..0x240 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo17(&self) -> &MO {
        &self.mo17
    }
    #[doc = "0x240..0x260 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo18(&self) -> &MO {
        &self.mo18
    }
    #[doc = "0x260..0x280 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo19(&self) -> &MO {
        &self.mo19
    }
    #[doc = "0x280..0x2a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo20(&self) -> &MO {
        &self.mo20
    }
    #[doc = "0x2a0..0x2c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo21(&self) -> &MO {
        &self.mo21
    }
    #[doc = "0x2c0..0x2e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo22(&self) -> &MO {
        &self.mo22
    }
    #[doc = "0x2e0..0x300 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo23(&self) -> &MO {
        &self.mo23
    }
    #[doc = "0x300..0x320 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo24(&self) -> &MO {
        &self.mo24
    }
    #[doc = "0x320..0x340 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo25(&self) -> &MO {
        &self.mo25
    }
    #[doc = "0x340..0x360 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo26(&self) -> &MO {
        &self.mo26
    }
    #[doc = "0x360..0x380 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo27(&self) -> &MO {
        &self.mo27
    }
    #[doc = "0x380..0x3a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo28(&self) -> &MO {
        &self.mo28
    }
    #[doc = "0x3a0..0x3c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo29(&self) -> &MO {
        &self.mo29
    }
    #[doc = "0x3c0..0x3e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo30(&self) -> &MO {
        &self.mo30
    }
    #[doc = "0x3e0..0x400 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo31(&self) -> &MO {
        &self.mo31
    }
    #[doc = "0x400..0x420 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo32(&self) -> &MO {
        &self.mo32
    }
    #[doc = "0x420..0x440 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo33(&self) -> &MO {
        &self.mo33
    }
    #[doc = "0x440..0x460 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo34(&self) -> &MO {
        &self.mo34
    }
    #[doc = "0x460..0x480 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo35(&self) -> &MO {
        &self.mo35
    }
    #[doc = "0x480..0x4a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo36(&self) -> &MO {
        &self.mo36
    }
    #[doc = "0x4a0..0x4c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo37(&self) -> &MO {
        &self.mo37
    }
    #[doc = "0x4c0..0x4e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo38(&self) -> &MO {
        &self.mo38
    }
    #[doc = "0x4e0..0x500 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo39(&self) -> &MO {
        &self.mo39
    }
    #[doc = "0x500..0x520 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo40(&self) -> &MO {
        &self.mo40
    }
    #[doc = "0x520..0x540 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo41(&self) -> &MO {
        &self.mo41
    }
    #[doc = "0x540..0x560 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo42(&self) -> &MO {
        &self.mo42
    }
    #[doc = "0x560..0x580 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo43(&self) -> &MO {
        &self.mo43
    }
    #[doc = "0x580..0x5a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo44(&self) -> &MO {
        &self.mo44
    }
    #[doc = "0x5a0..0x5c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo45(&self) -> &MO {
        &self.mo45
    }
    #[doc = "0x5c0..0x5e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo46(&self) -> &MO {
        &self.mo46
    }
    #[doc = "0x5e0..0x600 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo47(&self) -> &MO {
        &self.mo47
    }
    #[doc = "0x600..0x620 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo48(&self) -> &MO {
        &self.mo48
    }
    #[doc = "0x620..0x640 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo49(&self) -> &MO {
        &self.mo49
    }
    #[doc = "0x640..0x660 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo50(&self) -> &MO {
        &self.mo50
    }
    #[doc = "0x660..0x680 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo51(&self) -> &MO {
        &self.mo51
    }
    #[doc = "0x680..0x6a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo52(&self) -> &MO {
        &self.mo52
    }
    #[doc = "0x6a0..0x6c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo53(&self) -> &MO {
        &self.mo53
    }
    #[doc = "0x6c0..0x6e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo54(&self) -> &MO {
        &self.mo54
    }
    #[doc = "0x6e0..0x700 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo55(&self) -> &MO {
        &self.mo55
    }
    #[doc = "0x700..0x720 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo56(&self) -> &MO {
        &self.mo56
    }
    #[doc = "0x720..0x740 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo57(&self) -> &MO {
        &self.mo57
    }
    #[doc = "0x740..0x760 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo58(&self) -> &MO {
        &self.mo58
    }
    #[doc = "0x760..0x780 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo59(&self) -> &MO {
        &self.mo59
    }
    #[doc = "0x780..0x7a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo60(&self) -> &MO {
        &self.mo60
    }
    #[doc = "0x7a0..0x7c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo61(&self) -> &MO {
        &self.mo61
    }
    #[doc = "0x7c0..0x7e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo62(&self) -> &MO {
        &self.mo62
    }
    #[doc = "0x7e0..0x800 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo63(&self) -> &MO {
        &self.mo63
    }
    #[doc = "0x800..0x820 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo64(&self) -> &MO {
        &self.mo64
    }
    #[doc = "0x820..0x840 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo65(&self) -> &MO {
        &self.mo65
    }
    #[doc = "0x840..0x860 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo66(&self) -> &MO {
        &self.mo66
    }
    #[doc = "0x860..0x880 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo67(&self) -> &MO {
        &self.mo67
    }
    #[doc = "0x880..0x8a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo68(&self) -> &MO {
        &self.mo68
    }
    #[doc = "0x8a0..0x8c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo69(&self) -> &MO {
        &self.mo69
    }
    #[doc = "0x8c0..0x8e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo70(&self) -> &MO {
        &self.mo70
    }
    #[doc = "0x8e0..0x900 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo71(&self) -> &MO {
        &self.mo71
    }
    #[doc = "0x900..0x920 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo72(&self) -> &MO {
        &self.mo72
    }
    #[doc = "0x920..0x940 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo73(&self) -> &MO {
        &self.mo73
    }
    #[doc = "0x940..0x960 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo74(&self) -> &MO {
        &self.mo74
    }
    #[doc = "0x960..0x980 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo75(&self) -> &MO {
        &self.mo75
    }
    #[doc = "0x980..0x9a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo76(&self) -> &MO {
        &self.mo76
    }
    #[doc = "0x9a0..0x9c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo77(&self) -> &MO {
        &self.mo77
    }
    #[doc = "0x9c0..0x9e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo78(&self) -> &MO {
        &self.mo78
    }
    #[doc = "0x9e0..0xa00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo79(&self) -> &MO {
        &self.mo79
    }
    #[doc = "0xa00..0xa20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo80(&self) -> &MO {
        &self.mo80
    }
    #[doc = "0xa20..0xa40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo81(&self) -> &MO {
        &self.mo81
    }
    #[doc = "0xa40..0xa60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo82(&self) -> &MO {
        &self.mo82
    }
    #[doc = "0xa60..0xa80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo83(&self) -> &MO {
        &self.mo83
    }
    #[doc = "0xa80..0xaa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo84(&self) -> &MO {
        &self.mo84
    }
    #[doc = "0xaa0..0xac0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo85(&self) -> &MO {
        &self.mo85
    }
    #[doc = "0xac0..0xae0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo86(&self) -> &MO {
        &self.mo86
    }
    #[doc = "0xae0..0xb00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo87(&self) -> &MO {
        &self.mo87
    }
    #[doc = "0xb00..0xb20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo88(&self) -> &MO {
        &self.mo88
    }
    #[doc = "0xb20..0xb40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo89(&self) -> &MO {
        &self.mo89
    }
    #[doc = "0xb40..0xb60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo90(&self) -> &MO {
        &self.mo90
    }
    #[doc = "0xb60..0xb80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo91(&self) -> &MO {
        &self.mo91
    }
    #[doc = "0xb80..0xba0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo92(&self) -> &MO {
        &self.mo92
    }
    #[doc = "0xba0..0xbc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo93(&self) -> &MO {
        &self.mo93
    }
    #[doc = "0xbc0..0xbe0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo94(&self) -> &MO {
        &self.mo94
    }
    #[doc = "0xbe0..0xc00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo95(&self) -> &MO {
        &self.mo95
    }
    #[doc = "0xc00..0xc20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo96(&self) -> &MO {
        &self.mo96
    }
    #[doc = "0xc20..0xc40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo97(&self) -> &MO {
        &self.mo97
    }
    #[doc = "0xc40..0xc60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo98(&self) -> &MO {
        &self.mo98
    }
    #[doc = "0xc60..0xc80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo99(&self) -> &MO {
        &self.mo99
    }
    #[doc = "0xc80..0xca0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo100(&self) -> &MO {
        &self.mo100
    }
    #[doc = "0xca0..0xcc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo101(&self) -> &MO {
        &self.mo101
    }
    #[doc = "0xcc0..0xce0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo102(&self) -> &MO {
        &self.mo102
    }
    #[doc = "0xce0..0xd00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo103(&self) -> &MO {
        &self.mo103
    }
    #[doc = "0xd00..0xd20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo104(&self) -> &MO {
        &self.mo104
    }
    #[doc = "0xd20..0xd40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo105(&self) -> &MO {
        &self.mo105
    }
    #[doc = "0xd40..0xd60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo106(&self) -> &MO {
        &self.mo106
    }
    #[doc = "0xd60..0xd80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo107(&self) -> &MO {
        &self.mo107
    }
    #[doc = "0xd80..0xda0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo108(&self) -> &MO {
        &self.mo108
    }
    #[doc = "0xda0..0xdc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo109(&self) -> &MO {
        &self.mo109
    }
    #[doc = "0xdc0..0xde0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo110(&self) -> &MO {
        &self.mo110
    }
    #[doc = "0xde0..0xe00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo111(&self) -> &MO {
        &self.mo111
    }
    #[doc = "0xe00..0xe20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo112(&self) -> &MO {
        &self.mo112
    }
    #[doc = "0xe20..0xe40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo113(&self) -> &MO {
        &self.mo113
    }
    #[doc = "0xe40..0xe60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo114(&self) -> &MO {
        &self.mo114
    }
    #[doc = "0xe60..0xe80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo115(&self) -> &MO {
        &self.mo115
    }
    #[doc = "0xe80..0xea0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo116(&self) -> &MO {
        &self.mo116
    }
    #[doc = "0xea0..0xec0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo117(&self) -> &MO {
        &self.mo117
    }
    #[doc = "0xec0..0xee0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo118(&self) -> &MO {
        &self.mo118
    }
    #[doc = "0xee0..0xf00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo119(&self) -> &MO {
        &self.mo119
    }
    #[doc = "0xf00..0xf20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo120(&self) -> &MO {
        &self.mo120
    }
    #[doc = "0xf20..0xf40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo121(&self) -> &MO {
        &self.mo121
    }
    #[doc = "0xf40..0xf60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo122(&self) -> &MO {
        &self.mo122
    }
    #[doc = "0xf60..0xf80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo123(&self) -> &MO {
        &self.mo123
    }
    #[doc = "0xf80..0xfa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo124(&self) -> &MO {
        &self.mo124
    }
    #[doc = "0xfa0..0xfc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo125(&self) -> &MO {
        &self.mo125
    }
    #[doc = "0xfc0..0xfe0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo126(&self) -> &MO {
        &self.mo126
    }
    #[doc = "0xfe0..0x1000 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo127(&self) -> &MO {
        &self.mo127
    }
    #[doc = "0x1000..0x1020 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo128(&self) -> &MO {
        &self.mo128
    }
    #[doc = "0x1020..0x1040 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo129(&self) -> &MO {
        &self.mo129
    }
    #[doc = "0x1040..0x1060 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo130(&self) -> &MO {
        &self.mo130
    }
    #[doc = "0x1060..0x1080 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo131(&self) -> &MO {
        &self.mo131
    }
    #[doc = "0x1080..0x10a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo132(&self) -> &MO {
        &self.mo132
    }
    #[doc = "0x10a0..0x10c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo133(&self) -> &MO {
        &self.mo133
    }
    #[doc = "0x10c0..0x10e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo134(&self) -> &MO {
        &self.mo134
    }
    #[doc = "0x10e0..0x1100 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo135(&self) -> &MO {
        &self.mo135
    }
    #[doc = "0x1100..0x1120 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo136(&self) -> &MO {
        &self.mo136
    }
    #[doc = "0x1120..0x1140 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo137(&self) -> &MO {
        &self.mo137
    }
    #[doc = "0x1140..0x1160 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo138(&self) -> &MO {
        &self.mo138
    }
    #[doc = "0x1160..0x1180 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo139(&self) -> &MO {
        &self.mo139
    }
    #[doc = "0x1180..0x11a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo140(&self) -> &MO {
        &self.mo140
    }
    #[doc = "0x11a0..0x11c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo141(&self) -> &MO {
        &self.mo141
    }
    #[doc = "0x11c0..0x11e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo142(&self) -> &MO {
        &self.mo142
    }
    #[doc = "0x11e0..0x1200 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo143(&self) -> &MO {
        &self.mo143
    }
    #[doc = "0x1200..0x1220 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo144(&self) -> &MO {
        &self.mo144
    }
    #[doc = "0x1220..0x1240 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo145(&self) -> &MO {
        &self.mo145
    }
    #[doc = "0x1240..0x1260 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo146(&self) -> &MO {
        &self.mo146
    }
    #[doc = "0x1260..0x1280 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo147(&self) -> &MO {
        &self.mo147
    }
    #[doc = "0x1280..0x12a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo148(&self) -> &MO {
        &self.mo148
    }
    #[doc = "0x12a0..0x12c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo149(&self) -> &MO {
        &self.mo149
    }
    #[doc = "0x12c0..0x12e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo150(&self) -> &MO {
        &self.mo150
    }
    #[doc = "0x12e0..0x1300 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo151(&self) -> &MO {
        &self.mo151
    }
    #[doc = "0x1300..0x1320 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo152(&self) -> &MO {
        &self.mo152
    }
    #[doc = "0x1320..0x1340 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo153(&self) -> &MO {
        &self.mo153
    }
    #[doc = "0x1340..0x1360 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo154(&self) -> &MO {
        &self.mo154
    }
    #[doc = "0x1360..0x1380 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo155(&self) -> &MO {
        &self.mo155
    }
    #[doc = "0x1380..0x13a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo156(&self) -> &MO {
        &self.mo156
    }
    #[doc = "0x13a0..0x13c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo157(&self) -> &MO {
        &self.mo157
    }
    #[doc = "0x13c0..0x13e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo158(&self) -> &MO {
        &self.mo158
    }
    #[doc = "0x13e0..0x1400 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo159(&self) -> &MO {
        &self.mo159
    }
    #[doc = "0x1400..0x1420 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo160(&self) -> &MO {
        &self.mo160
    }
    #[doc = "0x1420..0x1440 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo161(&self) -> &MO {
        &self.mo161
    }
    #[doc = "0x1440..0x1460 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo162(&self) -> &MO {
        &self.mo162
    }
    #[doc = "0x1460..0x1480 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo163(&self) -> &MO {
        &self.mo163
    }
    #[doc = "0x1480..0x14a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo164(&self) -> &MO {
        &self.mo164
    }
    #[doc = "0x14a0..0x14c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo165(&self) -> &MO {
        &self.mo165
    }
    #[doc = "0x14c0..0x14e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo166(&self) -> &MO {
        &self.mo166
    }
    #[doc = "0x14e0..0x1500 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo167(&self) -> &MO {
        &self.mo167
    }
    #[doc = "0x1500..0x1520 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo168(&self) -> &MO {
        &self.mo168
    }
    #[doc = "0x1520..0x1540 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo169(&self) -> &MO {
        &self.mo169
    }
    #[doc = "0x1540..0x1560 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo170(&self) -> &MO {
        &self.mo170
    }
    #[doc = "0x1560..0x1580 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo171(&self) -> &MO {
        &self.mo171
    }
    #[doc = "0x1580..0x15a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo172(&self) -> &MO {
        &self.mo172
    }
    #[doc = "0x15a0..0x15c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo173(&self) -> &MO {
        &self.mo173
    }
    #[doc = "0x15c0..0x15e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo174(&self) -> &MO {
        &self.mo174
    }
    #[doc = "0x15e0..0x1600 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo175(&self) -> &MO {
        &self.mo175
    }
    #[doc = "0x1600..0x1620 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo176(&self) -> &MO {
        &self.mo176
    }
    #[doc = "0x1620..0x1640 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo177(&self) -> &MO {
        &self.mo177
    }
    #[doc = "0x1640..0x1660 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo178(&self) -> &MO {
        &self.mo178
    }
    #[doc = "0x1660..0x1680 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo179(&self) -> &MO {
        &self.mo179
    }
    #[doc = "0x1680..0x16a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo180(&self) -> &MO {
        &self.mo180
    }
    #[doc = "0x16a0..0x16c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo181(&self) -> &MO {
        &self.mo181
    }
    #[doc = "0x16c0..0x16e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo182(&self) -> &MO {
        &self.mo182
    }
    #[doc = "0x16e0..0x1700 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo183(&self) -> &MO {
        &self.mo183
    }
    #[doc = "0x1700..0x1720 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo184(&self) -> &MO {
        &self.mo184
    }
    #[doc = "0x1720..0x1740 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo185(&self) -> &MO {
        &self.mo185
    }
    #[doc = "0x1740..0x1760 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo186(&self) -> &MO {
        &self.mo186
    }
    #[doc = "0x1760..0x1780 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo187(&self) -> &MO {
        &self.mo187
    }
    #[doc = "0x1780..0x17a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo188(&self) -> &MO {
        &self.mo188
    }
    #[doc = "0x17a0..0x17c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo189(&self) -> &MO {
        &self.mo189
    }
    #[doc = "0x17c0..0x17e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo190(&self) -> &MO {
        &self.mo190
    }
    #[doc = "0x17e0..0x1800 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo191(&self) -> &MO {
        &self.mo191
    }
    #[doc = "0x1800..0x1820 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo192(&self) -> &MO {
        &self.mo192
    }
    #[doc = "0x1820..0x1840 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo193(&self) -> &MO {
        &self.mo193
    }
    #[doc = "0x1840..0x1860 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo194(&self) -> &MO {
        &self.mo194
    }
    #[doc = "0x1860..0x1880 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo195(&self) -> &MO {
        &self.mo195
    }
    #[doc = "0x1880..0x18a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo196(&self) -> &MO {
        &self.mo196
    }
    #[doc = "0x18a0..0x18c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo197(&self) -> &MO {
        &self.mo197
    }
    #[doc = "0x18c0..0x18e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo198(&self) -> &MO {
        &self.mo198
    }
    #[doc = "0x18e0..0x1900 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo199(&self) -> &MO {
        &self.mo199
    }
    #[doc = "0x1900..0x1920 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo200(&self) -> &MO {
        &self.mo200
    }
    #[doc = "0x1920..0x1940 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo201(&self) -> &MO {
        &self.mo201
    }
    #[doc = "0x1940..0x1960 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo202(&self) -> &MO {
        &self.mo202
    }
    #[doc = "0x1960..0x1980 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo203(&self) -> &MO {
        &self.mo203
    }
    #[doc = "0x1980..0x19a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo204(&self) -> &MO {
        &self.mo204
    }
    #[doc = "0x19a0..0x19c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo205(&self) -> &MO {
        &self.mo205
    }
    #[doc = "0x19c0..0x19e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo206(&self) -> &MO {
        &self.mo206
    }
    #[doc = "0x19e0..0x1a00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo207(&self) -> &MO {
        &self.mo207
    }
    #[doc = "0x1a00..0x1a20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo208(&self) -> &MO {
        &self.mo208
    }
    #[doc = "0x1a20..0x1a40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo209(&self) -> &MO {
        &self.mo209
    }
    #[doc = "0x1a40..0x1a60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo210(&self) -> &MO {
        &self.mo210
    }
    #[doc = "0x1a60..0x1a80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo211(&self) -> &MO {
        &self.mo211
    }
    #[doc = "0x1a80..0x1aa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo212(&self) -> &MO {
        &self.mo212
    }
    #[doc = "0x1aa0..0x1ac0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo213(&self) -> &MO {
        &self.mo213
    }
    #[doc = "0x1ac0..0x1ae0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo214(&self) -> &MO {
        &self.mo214
    }
    #[doc = "0x1ae0..0x1b00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo215(&self) -> &MO {
        &self.mo215
    }
    #[doc = "0x1b00..0x1b20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo216(&self) -> &MO {
        &self.mo216
    }
    #[doc = "0x1b20..0x1b40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo217(&self) -> &MO {
        &self.mo217
    }
    #[doc = "0x1b40..0x1b60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo218(&self) -> &MO {
        &self.mo218
    }
    #[doc = "0x1b60..0x1b80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo219(&self) -> &MO {
        &self.mo219
    }
    #[doc = "0x1b80..0x1ba0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo220(&self) -> &MO {
        &self.mo220
    }
    #[doc = "0x1ba0..0x1bc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo221(&self) -> &MO {
        &self.mo221
    }
    #[doc = "0x1bc0..0x1be0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo222(&self) -> &MO {
        &self.mo222
    }
    #[doc = "0x1be0..0x1c00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo223(&self) -> &MO {
        &self.mo223
    }
    #[doc = "0x1c00..0x1c20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo224(&self) -> &MO {
        &self.mo224
    }
    #[doc = "0x1c20..0x1c40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo225(&self) -> &MO {
        &self.mo225
    }
    #[doc = "0x1c40..0x1c60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo226(&self) -> &MO {
        &self.mo226
    }
    #[doc = "0x1c60..0x1c80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo227(&self) -> &MO {
        &self.mo227
    }
    #[doc = "0x1c80..0x1ca0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo228(&self) -> &MO {
        &self.mo228
    }
    #[doc = "0x1ca0..0x1cc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo229(&self) -> &MO {
        &self.mo229
    }
    #[doc = "0x1cc0..0x1ce0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo230(&self) -> &MO {
        &self.mo230
    }
    #[doc = "0x1ce0..0x1d00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo231(&self) -> &MO {
        &self.mo231
    }
    #[doc = "0x1d00..0x1d20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo232(&self) -> &MO {
        &self.mo232
    }
    #[doc = "0x1d20..0x1d40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo233(&self) -> &MO {
        &self.mo233
    }
    #[doc = "0x1d40..0x1d60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo234(&self) -> &MO {
        &self.mo234
    }
    #[doc = "0x1d60..0x1d80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo235(&self) -> &MO {
        &self.mo235
    }
    #[doc = "0x1d80..0x1da0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo236(&self) -> &MO {
        &self.mo236
    }
    #[doc = "0x1da0..0x1dc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo237(&self) -> &MO {
        &self.mo237
    }
    #[doc = "0x1dc0..0x1de0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo238(&self) -> &MO {
        &self.mo238
    }
    #[doc = "0x1de0..0x1e00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo239(&self) -> &MO {
        &self.mo239
    }
    #[doc = "0x1e00..0x1e20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo240(&self) -> &MO {
        &self.mo240
    }
    #[doc = "0x1e20..0x1e40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo241(&self) -> &MO {
        &self.mo241
    }
    #[doc = "0x1e40..0x1e60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo242(&self) -> &MO {
        &self.mo242
    }
    #[doc = "0x1e60..0x1e80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo243(&self) -> &MO {
        &self.mo243
    }
    #[doc = "0x1e80..0x1ea0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo244(&self) -> &MO {
        &self.mo244
    }
    #[doc = "0x1ea0..0x1ec0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo245(&self) -> &MO {
        &self.mo245
    }
    #[doc = "0x1ec0..0x1ee0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo246(&self) -> &MO {
        &self.mo246
    }
    #[doc = "0x1ee0..0x1f00 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo247(&self) -> &MO {
        &self.mo247
    }
    #[doc = "0x1f00..0x1f20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo248(&self) -> &MO {
        &self.mo248
    }
    #[doc = "0x1f20..0x1f40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo249(&self) -> &MO {
        &self.mo249
    }
    #[doc = "0x1f40..0x1f60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo250(&self) -> &MO {
        &self.mo250
    }
    #[doc = "0x1f60..0x1f80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo251(&self) -> &MO {
        &self.mo251
    }
    #[doc = "0x1f80..0x1fa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo252(&self) -> &MO {
        &self.mo252
    }
    #[doc = "0x1fa0..0x1fc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo253(&self) -> &MO {
        &self.mo253
    }
    #[doc = "0x1fc0..0x1fe0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo254(&self) -> &MO {
        &self.mo254
    }
    #[doc = "0x1fe0..0x2000 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo255(&self) -> &MO {
        &self.mo255
    }
}
#[doc = "Message Object Registers"]
pub use self::mo::MO;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;
