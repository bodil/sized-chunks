// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Helper types for chunks.

use std::fmt::Debug;
use std::marker::PhantomData;

use typenum::*;

// Chunk sizes

/// A trait used to decide the size of an array.
///
/// `<N as ChunkLength<A>>::SizedType` for a type level integer N will have the
/// same size as `[A; N]`.
pub trait ChunkLength<A>: Unsigned {
    type SizedType;
}

impl<A> ChunkLength<A> for UTerm {
    type SizedType = ();
}

#[doc(hidden)]
#[allow(dead_code)]
pub struct SizeEven<A, B> {
    parent1: B,
    parent2: B,
    _marker: PhantomData<A>,
}

#[doc(hidden)]
#[allow(dead_code)]
pub struct SizeOdd<A, B> {
    parent1: B,
    parent2: B,
    data: A,
}

impl<A, N> ChunkLength<A> for UInt<N, B0>
where
    N: ChunkLength<A>,
{
    type SizedType = SizeEven<A, N::SizedType>;
}

impl<A, N> ChunkLength<A> for UInt<N, B1>
where
    N: ChunkLength<A>,
{
    type SizedType = SizeOdd<A, N::SizedType>;
}

// Bit field sizes

/// A type level number signifying the number of bits in a bitmap.
///
/// This trait is implemented for type level numbers from `U1` to `U128`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate sized_chunks;
/// # extern crate typenum;
/// # use sized_chunks::types::Bits;
/// # use typenum::U10;
/// # fn main() {
/// assert_eq!(
///     std::mem::size_of::<<U10 as Bits>::Store>(),
///     std::mem::size_of::<u16>()
/// );
/// # }
/// ```
pub trait Bits: Unsigned {
    /// A primitive integer type suitable for storing this many bits.
    type Store: Default + Copy + PartialEq + Debug;

    fn get(bits: &Self::Store, index: usize) -> bool;
    fn set(bits: &mut Self::Store, index: usize, value: bool) -> bool;
    fn len(bits: &Self::Store) -> usize;
    fn first_index(bits: &Self::Store) -> Option<usize>;
}

macro_rules! bits_for {
    ($num:ty, $result:ty) => {
        impl Bits for $num {
            type Store = $result;

            fn get(bits: &$result, index: usize) -> bool {
                debug_assert!(index < Self::USIZE);
                bits & (1 << index) != 0
            }

            fn set(bits: &mut $result, index: usize, value: bool) -> bool {
                debug_assert!(index < Self::USIZE);
                let mask = 1 << index;
                let prev = *bits & mask;
                if value {
                    *bits |= mask;
                } else {
                    *bits &= !mask;
                }
                prev != 0
            }

            fn len(bits: &$result) -> usize {
                bits.count_ones() as usize
            }

            fn first_index(bits: &$result) -> Option<usize> {
                if *bits == 0 {
                    None
                } else {
                    Some(bits.trailing_zeros() as usize)
                }
            }
        }
    };
}

macro_rules! bits_for_256 {
    ($num:ty) => {
        impl Bits for $num {
            type Store = [u128; 2];

            fn get(bits: &Self::Store, index: usize) -> bool {
                debug_assert!(index < Self::USIZE);
                if index < 128 {
                    bits[0] & (1 << index) != 0
                } else {
                    bits[1] & (1 << (index - 128)) != 0
                }
            }

            fn set(bits: &mut Self::Store, index: usize, value: bool) -> bool {
                debug_assert!(index < Self::USIZE);
                let mask = 1 << (index & 127);
                let bits = if index < 128 {
                    &mut bits[0]
                } else {
                    &mut bits[1]
                };
                let prev = *bits & mask;
                if value {
                    *bits |= mask;
                } else {
                    *bits &= !mask;
                }
                prev != 0
            }

            fn len(bits: &Self::Store) -> usize {
                (bits[0].count_ones() + bits[1].count_ones()) as usize
            }

            fn first_index(bits: &Self::Store) -> Option<usize> {
                if bits[0] == 0 {
                    if bits[1] == 0 {
                        None
                    } else {
                        Some(bits[1].trailing_zeros() as usize + 128)
                    }
                } else {
                    Some(bits[0].trailing_zeros() as usize)
                }
            }
        }
    };
}

bits_for!(U1, u8);
bits_for!(U2, u8);
bits_for!(U3, u8);
bits_for!(U4, u8);
bits_for!(U5, u8);
bits_for!(U6, u8);
bits_for!(U7, u8);
bits_for!(U8, u8);
bits_for!(U9, u16);
bits_for!(U10, u16);
bits_for!(U11, u16);
bits_for!(U12, u16);
bits_for!(U13, u16);
bits_for!(U14, u16);
bits_for!(U15, u16);
bits_for!(U16, u16);
bits_for!(U17, u32);
bits_for!(U18, u32);
bits_for!(U19, u32);
bits_for!(U20, u32);
bits_for!(U21, u32);
bits_for!(U22, u32);
bits_for!(U23, u32);
bits_for!(U24, u32);
bits_for!(U25, u32);
bits_for!(U26, u32);
bits_for!(U27, u32);
bits_for!(U28, u32);
bits_for!(U29, u32);
bits_for!(U30, u32);
bits_for!(U31, u32);
bits_for!(U32, u32);
bits_for!(U33, u64);
bits_for!(U34, u64);
bits_for!(U35, u64);
bits_for!(U36, u64);
bits_for!(U37, u64);
bits_for!(U38, u64);
bits_for!(U39, u64);
bits_for!(U40, u64);
bits_for!(U41, u64);
bits_for!(U42, u64);
bits_for!(U43, u64);
bits_for!(U44, u64);
bits_for!(U45, u64);
bits_for!(U46, u64);
bits_for!(U47, u64);
bits_for!(U48, u64);
bits_for!(U49, u64);
bits_for!(U50, u64);
bits_for!(U51, u64);
bits_for!(U52, u64);
bits_for!(U53, u64);
bits_for!(U54, u64);
bits_for!(U55, u64);
bits_for!(U56, u64);
bits_for!(U57, u64);
bits_for!(U58, u64);
bits_for!(U59, u64);
bits_for!(U60, u64);
bits_for!(U61, u64);
bits_for!(U62, u64);
bits_for!(U63, u64);
bits_for!(U64, u64);
bits_for!(U65, u128);
bits_for!(U66, u128);
bits_for!(U67, u128);
bits_for!(U68, u128);
bits_for!(U69, u128);
bits_for!(U70, u128);
bits_for!(U71, u128);
bits_for!(U72, u128);
bits_for!(U73, u128);
bits_for!(U74, u128);
bits_for!(U75, u128);
bits_for!(U76, u128);
bits_for!(U77, u128);
bits_for!(U78, u128);
bits_for!(U79, u128);
bits_for!(U80, u128);
bits_for!(U81, u128);
bits_for!(U82, u128);
bits_for!(U83, u128);
bits_for!(U84, u128);
bits_for!(U85, u128);
bits_for!(U86, u128);
bits_for!(U87, u128);
bits_for!(U88, u128);
bits_for!(U89, u128);
bits_for!(U90, u128);
bits_for!(U91, u128);
bits_for!(U92, u128);
bits_for!(U93, u128);
bits_for!(U94, u128);
bits_for!(U95, u128);
bits_for!(U96, u128);
bits_for!(U97, u128);
bits_for!(U98, u128);
bits_for!(U99, u128);
bits_for!(U100, u128);
bits_for!(U101, u128);
bits_for!(U102, u128);
bits_for!(U103, u128);
bits_for!(U104, u128);
bits_for!(U105, u128);
bits_for!(U106, u128);
bits_for!(U107, u128);
bits_for!(U108, u128);
bits_for!(U109, u128);
bits_for!(U110, u128);
bits_for!(U111, u128);
bits_for!(U112, u128);
bits_for!(U113, u128);
bits_for!(U114, u128);
bits_for!(U115, u128);
bits_for!(U116, u128);
bits_for!(U117, u128);
bits_for!(U118, u128);
bits_for!(U119, u128);
bits_for!(U120, u128);
bits_for!(U121, u128);
bits_for!(U122, u128);
bits_for!(U123, u128);
bits_for!(U124, u128);
bits_for!(U125, u128);
bits_for!(U126, u128);
bits_for!(U127, u128);
bits_for!(U128, u128);
bits_for_256!(U129);
bits_for_256!(U130);
bits_for_256!(U131);
bits_for_256!(U132);
bits_for_256!(U133);
bits_for_256!(U134);
bits_for_256!(U135);
bits_for_256!(U136);
bits_for_256!(U137);
bits_for_256!(U138);
bits_for_256!(U139);
bits_for_256!(U140);
bits_for_256!(U141);
bits_for_256!(U142);
bits_for_256!(U143);
bits_for_256!(U144);
bits_for_256!(U145);
bits_for_256!(U146);
bits_for_256!(U147);
bits_for_256!(U148);
bits_for_256!(U149);
bits_for_256!(U150);
bits_for_256!(U151);
bits_for_256!(U152);
bits_for_256!(U153);
bits_for_256!(U154);
bits_for_256!(U155);
bits_for_256!(U156);
bits_for_256!(U157);
bits_for_256!(U158);
bits_for_256!(U159);
bits_for_256!(U160);
bits_for_256!(U161);
bits_for_256!(U162);
bits_for_256!(U163);
bits_for_256!(U164);
bits_for_256!(U165);
bits_for_256!(U166);
bits_for_256!(U167);
bits_for_256!(U168);
bits_for_256!(U169);
bits_for_256!(U170);
bits_for_256!(U171);
bits_for_256!(U172);
bits_for_256!(U173);
bits_for_256!(U174);
bits_for_256!(U175);
bits_for_256!(U176);
bits_for_256!(U177);
bits_for_256!(U178);
bits_for_256!(U179);
bits_for_256!(U180);
bits_for_256!(U181);
bits_for_256!(U182);
bits_for_256!(U183);
bits_for_256!(U184);
bits_for_256!(U185);
bits_for_256!(U186);
bits_for_256!(U187);
bits_for_256!(U188);
bits_for_256!(U189);
bits_for_256!(U190);
bits_for_256!(U191);
bits_for_256!(U192);
bits_for_256!(U193);
bits_for_256!(U194);
bits_for_256!(U195);
bits_for_256!(U196);
bits_for_256!(U197);
bits_for_256!(U198);
bits_for_256!(U199);
bits_for_256!(U200);
bits_for_256!(U201);
bits_for_256!(U202);
bits_for_256!(U203);
bits_for_256!(U204);
bits_for_256!(U205);
bits_for_256!(U206);
bits_for_256!(U207);
bits_for_256!(U208);
bits_for_256!(U209);
bits_for_256!(U210);
bits_for_256!(U211);
bits_for_256!(U212);
bits_for_256!(U213);
bits_for_256!(U214);
bits_for_256!(U215);
bits_for_256!(U216);
bits_for_256!(U217);
bits_for_256!(U218);
bits_for_256!(U219);
bits_for_256!(U220);
bits_for_256!(U221);
bits_for_256!(U222);
bits_for_256!(U223);
bits_for_256!(U224);
bits_for_256!(U225);
bits_for_256!(U226);
bits_for_256!(U227);
bits_for_256!(U228);
bits_for_256!(U229);
bits_for_256!(U230);
bits_for_256!(U231);
bits_for_256!(U232);
bits_for_256!(U233);
bits_for_256!(U234);
bits_for_256!(U235);
bits_for_256!(U236);
bits_for_256!(U237);
bits_for_256!(U238);
bits_for_256!(U239);
bits_for_256!(U240);
bits_for_256!(U241);
bits_for_256!(U242);
bits_for_256!(U243);
bits_for_256!(U244);
bits_for_256!(U245);
bits_for_256!(U246);
bits_for_256!(U247);
bits_for_256!(U248);
bits_for_256!(U249);
bits_for_256!(U250);
bits_for_256!(U251);
bits_for_256!(U252);
bits_for_256!(U253);
bits_for_256!(U254);
bits_for_256!(U255);
bits_for_256!(U256);
