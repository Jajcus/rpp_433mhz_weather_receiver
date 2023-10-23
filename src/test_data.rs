use crate::radio::Pulse;
use crate::radio::PulseKind::{High, Low, Reset};

pub const PROLOGUE_TEST_STREAM_L: usize = 545;
pub const PROLOGUE_TEST_STREAM: [Pulse; 545] = [
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 366,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 933,
    },
    Pulse {
        kind: Low,
        length: 107,
    },
    Pulse {
        kind: High,
        length: 297,
    },
    Pulse {
        kind: Low,
        length: 93,
    },
    Pulse {
        kind: High,
        length: 154,
    },
    Pulse {
        kind: Low,
        length: 79,
    },
    Pulse {
        kind: High,
        length: 432,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 146,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 1076,
    },
    Pulse {
        kind: Low,
        length: 155,
    },
    Pulse {
        kind: High,
        length: 600,
    },
    Pulse {
        kind: Low,
        length: 4192,
    },
    Pulse {
        kind: High,
        length: 565,
    },
    Pulse {
        kind: Low,
        length: 2091,
    },
    Pulse {
        kind: High,
        length: 548,
    },
    Pulse {
        kind: Low,
        length: 2105,
    },
    Pulse {
        kind: High,
        length: 532,
    },
    Pulse {
        kind: Low,
        length: 4284,
    },
    Pulse {
        kind: High,
        length: 530,
    },
    Pulse {
        kind: Low,
        length: 4248,
    },
    Pulse {
        kind: High,
        length: 536,
    },
    Pulse {
        kind: Low,
        length: 2113,
    },
    Pulse {
        kind: High,
        length: 536,
    },
    Pulse {
        kind: Low,
        length: 4234,
    },
    Pulse {
        kind: High,
        length: 545,
    },
    Pulse {
        kind: Low,
        length: 4267,
    },
    Pulse {
        kind: High,
        length: 551,
    },
    Pulse {
        kind: Low,
        length: 4224,
    },
    Pulse {
        kind: High,
        length: 557,
    },
    Pulse {
        kind: Low,
        length: 2091,
    },
    Pulse {
        kind: High,
        length: 555,
    },
    Pulse {
        kind: Low,
        length: 4218,
    },
    Pulse {
        kind: High,
        length: 564,
    },
    Pulse {
        kind: Low,
        length: 4249,
    },
    Pulse {
        kind: High,
        length: 565,
    },
    Pulse {
        kind: Low,
        length: 2084,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 2082,
    },
    Pulse {
        kind: High,
        length: 565,
    },
    Pulse {
        kind: Low,
        length: 2081,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 2120,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 2081,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 2082,
    },
    Pulse {
        kind: High,
        length: 565,
    },
    Pulse {
        kind: Low,
        length: 2082,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 2120,
    },
    Pulse {
        kind: High,
        length: 565,
    },
    Pulse {
        kind: Low,
        length: 4212,
    },
    Pulse {
        kind: High,
        length: 567,
    },
    Pulse {
        kind: Low,
        length: 4206,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2111,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2108,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4237,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4198,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4230,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 9381,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4197,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2069,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4236,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2068,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4235,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4197,
    },
    Pulse {
        kind: High,
        length: 583,
    },
    Pulse {
        kind: Low,
        length: 2068,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4196,
    },
    Pulse {
        kind: High,
        length: 583,
    },
    Pulse {
        kind: Low,
        length: 4233,
    },
    Pulse {
        kind: High,
        length: 583,
    },
    Pulse {
        kind: Low,
        length: 2068,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2110,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2077,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 570,
    },
    Pulse {
        kind: Low,
        length: 2079,
    },
    Pulse {
        kind: High,
        length: 572,
    },
    Pulse {
        kind: Low,
        length: 2115,
    },
    Pulse {
        kind: High,
        length: 572,
    },
    Pulse {
        kind: Low,
        length: 4205,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4205,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2111,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2109,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4241,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4232,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2080,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 9386,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2069,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4239,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2069,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4239,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4197,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4239,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2068,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2110,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 2076,
    },
    Pulse {
        kind: High,
        length: 571,
    },
    Pulse {
        kind: Low,
        length: 2240,
    },
    Pulse {
        kind: High,
        length: 570,
    },
    Pulse {
        kind: Low,
        length: 4206,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4205,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2111,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2109,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4240,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2076,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4233,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2077,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 9386,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 582,
    },
    Pulse {
        kind: Low,
        length: 2069,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4239,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4236,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4238,
    },
    Pulse {
        kind: High,
        length: 582,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2112,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 2079,
    },
    Pulse {
        kind: High,
        length: 571,
    },
    Pulse {
        kind: Low,
        length: 2077,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2077,
    },
    Pulse {
        kind: High,
        length: 572,
    },
    Pulse {
        kind: Low,
        length: 2118,
    },
    Pulse {
        kind: High,
        length: 569,
    },
    Pulse {
        kind: Low,
        length: 4208,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2113,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4207,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2108,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4240,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4235,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2079,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 9386,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4242,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4242,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4200,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4238,
    },
    Pulse {
        kind: High,
        length: 582,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2111,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2078,
    },
    Pulse {
        kind: High,
        length: 571,
    },
    Pulse {
        kind: Low,
        length: 2079,
    },
    Pulse {
        kind: High,
        length: 572,
    },
    Pulse {
        kind: Low,
        length: 2116,
    },
    Pulse {
        kind: High,
        length: 571,
    },
    Pulse {
        kind: Low,
        length: 4210,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 4205,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2111,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4205,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2107,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4203,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4240,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 575,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4234,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2080,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 9389,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 2071,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2072,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4240,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2073,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 4238,
    },
    Pulse {
        kind: High,
        length: 582,
    },
    Pulse {
        kind: Low,
        length: 4199,
    },
    Pulse {
        kind: High,
        length: 584,
    },
    Pulse {
        kind: Low,
        length: 2068,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4202,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 4238,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 581,
    },
    Pulse {
        kind: Low,
        length: 2070,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2115,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2076,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 2078,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 2076,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 2116,
    },
    Pulse {
        kind: High,
        length: 572,
    },
    Pulse {
        kind: Low,
        length: 4208,
    },
    Pulse {
        kind: High,
        length: 574,
    },
    Pulse {
        kind: Low,
        length: 4207,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2113,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4204,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4201,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 2112,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 2076,
    },
    Pulse {
        kind: High,
        length: 573,
    },
    Pulse {
        kind: Low,
        length: 4206,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 4240,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 2075,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2074,
    },
    Pulse {
        kind: High,
        length: 576,
    },
    Pulse {
        kind: Low,
        length: 4324,
    },
    Pulse {
        kind: High,
        length: 579,
    },
    Pulse {
        kind: Low,
        length: 4233,
    },
    Pulse {
        kind: High,
        length: 578,
    },
    Pulse {
        kind: Low,
        length: 2067,
    },
    Pulse {
        kind: High,
        length: 577,
    },
    Pulse {
        kind: Low,
        length: 9387,
    },
    Pulse {
        kind: High,
        length: 580,
    },
    Pulse {
        kind: Low,
        length: 361992,
    },
    Pulse {
        kind: High,
        length: 58,
    },
    Pulse {
        kind: Low,
        length: 463,
    },
    Pulse {
        kind: High,
        length: 138,
    },
    Pulse {
        kind: Low,
        length: 125,
    },
    Pulse {
        kind: High,
        length: 165,
    },
    Pulse {
        kind: Low,
        length: 279,
    },
    Pulse {
        kind: High,
        length: 192,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: Low,
        length: 1471,
    },
    Pulse {
        kind: High,
        length: 61,
    },
    Pulse {
        kind: Low,
        length: 89,
    },
    Pulse {
        kind: High,
        length: 118,
    },
    Pulse {
        kind: Low,
        length: 686,
    },
    Pulse {
        kind: High,
        length: 241,
    },
    Pulse {
        kind: Low,
        length: 69,
    },
    Pulse {
        kind: High,
        length: 159,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 137,
    },
    Pulse {
        kind: Low,
        length: 213,
    },
    Pulse {
        kind: High,
        length: 136,
    },
    Pulse {
        kind: Low,
        length: 360,
    },
    Pulse {
        kind: High,
        length: 73,
    },
    Pulse {
        kind: Low,
        length: 65,
    },
    Pulse {
        kind: High,
        length: 63,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: Low,
        length: 288,
    },
    Pulse {
        kind: High,
        length: 63,
    },
    Pulse {
        kind: Low,
        length: 259,
    },
    Pulse {
        kind: High,
        length: 88,
    },
    Pulse {
        kind: Low,
        length: 183,
    },
    Pulse {
        kind: High,
        length: 447,
    },
    Pulse {
        kind: Low,
        length: 264,
    },
    Pulse {
        kind: High,
        length: 69,
    },
    Pulse {
        kind: Low,
        length: 253,
    },
    Pulse {
        kind: High,
        length: 1250,
    },
    Pulse {
        kind: Low,
        length: 245,
    },
    Pulse {
        kind: High,
        length: 649,
    },
    Pulse {
        kind: Low,
        length: 100,
    },
    Pulse {
        kind: High,
        length: 493,
    },
    Pulse {
        kind: Low,
        length: 53,
    },
    Pulse {
        kind: High,
        length: 131,
    },
    Pulse {
        kind: Low,
        length: 106,
    },
    Pulse {
        kind: High,
        length: 1904,
    },
    Pulse {
        kind: Low,
        length: 310,
    },
    Pulse {
        kind: High,
        length: 657,
    },
    Pulse {
        kind: Low,
        length: 93,
    },
    Pulse {
        kind: High,
        length: 305,
    },
    Pulse {
        kind: Low,
        length: 196,
    },
    Pulse {
        kind: High,
        length: 1114,
    },
    Pulse {
        kind: Low,
        length: 60,
    },
    Pulse {
        kind: High,
        length: 391,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 132,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 403,
    },
    Pulse {
        kind: Low,
        length: 182,
    },
    Pulse {
        kind: High,
        length: 201,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 1295,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 1244,
    },
    Pulse {
        kind: Low,
        length: 343,
    },
    Pulse {
        kind: High,
        length: 1826,
    },
    Pulse {
        kind: Low,
        length: 60,
    },
    Pulse {
        kind: High,
        length: 67,
    },
    Pulse {
        kind: Reset,
        length: 0,
    },
    Pulse {
        kind: High,
        length: 81,
    },
    Pulse {
        kind: Low,
        length: 143,
    },
    Pulse {
        kind: High,
        length: 1711,
    },
    Pulse {
        kind: Low,
        length: 206,
    },
];
