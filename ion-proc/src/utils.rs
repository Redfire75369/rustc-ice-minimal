/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use prettyplease::unparse;
use proc_macro2::Ident;
use syn::{GenericArgument, parse2, Pat, PathArguments, Type, TypePath, TypeReference};
use syn::punctuated::Punctuated;
use syn::visit_mut::{visit_type_path_mut, visit_type_reference_mut, VisitMut};

pub(crate) fn type_ends_with<I: ?Sized>(ty: &TypePath, ident: &I) -> bool
where
	Ident: PartialEq<I>,
{
	if let Some(last) = ty.path.segments.last() {
		&last.ident == ident
	} else {
		false
	}
}

pub(crate) fn extract_type_argument(ty: &TypePath, index: usize) -> Option<Box<Type>> {
	if !ty.path.segments.is_empty() && ty.path.segments.len() > index {
		let last = ty.path.segments.last().unwrap();
		if let PathArguments::AngleBracketed(angle_bracketed) = &last.arguments {
			if let Some(GenericArgument::Type(ty)) = angle_bracketed.args.iter().nth(index) {
				return Some(Box::new(ty.clone()));
			}
		}
	}
	None
}

pub struct LifetimeRemover;

impl VisitMut for LifetimeRemover {
	fn visit_type_path_mut(&mut self, ty: &mut TypePath) {
		if let Some(segment) = ty.path.segments.last_mut() {
			if let PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
				arguments.args = Punctuated::from_iter(arguments.args.clone().into_iter().filter(|argument| {
					match argument {
						GenericArgument::Lifetime(lt) => *lt == parse_quote!('static),
						_ => true
					}
				}));
			}
		}
		visit_type_path_mut(self,ty);
	}

	fn visit_type_reference_mut(&mut self, ty: &mut TypeReference) {
		if ty.lifetime != Some(parse_quote!('static)) {
			ty.lifetime = None;
		}
		visit_type_reference_mut(self, ty);
	}
}

pub(crate) fn format_pat(pat: &Pat) -> Option<String> {
	let ident = match pat {
		Pat::Ident(ident) => ident.ident.clone(),
		_ => return None,
	};
	let pat = unparse(
		&parse2(quote!(
			const #ident: () = ();
		))
		.unwrap(),
	);
	let mut pat = String::from(pat.trim());
	pat.drain((pat.len() - 10)..(pat.len()));
	pat.drain(0..5);
	Some(String::from(pat.trim()))
}
