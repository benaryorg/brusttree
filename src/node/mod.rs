//! This is the implementation of a _node_.
//!
//! A node should manage the data inside it and offer useful methods to the
//! caller so when the `Tree` wants to store something in this node, it should
//! first check if it is possible to do this.
//!

use std::rc::Rc;

/// The default size for a new node created without parameters.
const DEFAULT_SIZE:usize=4;

/// The actual node.
pub struct Node<'a,T:'a+Sized+PartialOrd>
{
	/// The object which is saved internally.
	pub data:Rc<[Option<&'a T>]>,
}

/// Implementation of the `Node`.
impl<'a,T:'a+Sized+PartialOrd> Node<'a,T>
{
	/// The default constructor without parameters.
	///
	/// Here a new Node with a nodesize of `DEFAULT_SIZE` is created.
	pub fn new()->Self
	{
		Node
		{
			data:Rc::new([None;DEFAULT_SIZE]),
		}
	}
}

