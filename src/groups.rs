use std::rc::{Rc, Weak};
use std::slice::IterMut;

use window::Window;
use x::{Connection, WindowId};


pub struct Group {
    connection: Rc<Connection>,
    stack: Vec<Rc<WindowId>>,
    focus: Option<Weak<WindowId>>,
}

impl Group {
    pub fn new(connection: Rc<Connection>) -> Group {
        Group {
            connection: connection,
            stack: Vec::new(),
            focus: None,
        }
    }

    pub fn add_window(&mut self, window_id: WindowId) {
        self.stack.push(Rc::new(window_id));
    }

    pub fn find_window_by_id<'a>(&'a mut self, window_id: &WindowId) -> Option<GroupWindow<'a>> {
        self.stack
            .iter()
            .find(|rc| rc.as_ref() == window_id)
            .map(|rc| rc.clone())
            .map(move |rc| {
                     GroupWindow {
                         group: self,
                         window_id: rc,
                     }
                 })
    }

    pub fn get_focused<'a>(&'a mut self) -> Option<GroupWindow<'a>> {
        self.focus
            .clone()
            .and_then(|rc| rc.upgrade())
            .map(move |window_id| {
                     GroupWindow {
                         group: self,
                         window_id: window_id,
                     }
                 })
    }

    pub fn iter_mut<'a>(&'a mut self) -> GroupIter<'a> {
        GroupIter {
            connection: &self.connection,
            inner: self.stack.iter_mut(),
        }
    }
}


pub struct GroupWindow<'a> {
    group: &'a mut Group,
    window_id: Rc<WindowId>,
}

impl<'a> GroupWindow<'a> {
    pub fn remove_from_group(self) {
        let window_id = self.window_id.clone();
        self.group.stack.retain(|w| w != &window_id)
    }

    pub fn focus(&mut self) {
        self.group.focus = Some(Rc::downgrade(&self.window_id));
    }
}

impl<'a> Window for GroupWindow<'a> {
    fn connection(&self) -> &Connection {
        &self.group.connection
    }

    fn id(&self) -> &WindowId {
        self.window_id.as_ref()
    }
}


pub struct GroupIter<'a> {
    connection: &'a Connection,
    inner: IterMut<'a, Rc<WindowId>>,
}

impl<'a> Iterator for GroupIter<'a> {
    type Item = GroupIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|window_id| {
                                  GroupIterItem {
                                      connection: self.connection,
                                      window_id: window_id,
                                  }
                              })
    }
}

impl<'a> ExactSizeIterator for GroupIter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}


pub struct GroupIterItem<'a> {
    connection: &'a Connection,
    window_id: &'a WindowId,
}

impl<'a> Window for GroupIterItem<'a> {
    fn connection(&self) -> &Connection {
        self.connection
    }

    fn id(&self) -> &WindowId {
        self.window_id
    }
}
