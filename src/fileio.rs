use bytes::{
    Bytes
};
use crate::{
    AppIO,
    CCLang::{
        self,
        Binary,
        Handle,
        Index,
        Mode,
        Text,
        Whence
    },
    Machine,
    Mode as FileMode
};
use std::{
    any::Any,
    clone::Clone,
    cmp::{
        Ordering,
        PartialEq,
        PartialOrd
    },
    fs::{
        self,
        File,
        OpenOptions
    },
    io::{
        self,
        BufRead,
        BufReader,
        Seek,
        SeekFrom,
        Write
    },
    path::PathBuf,
    rc::Rc,
};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct FileIO;

#[derive(Clone)]
pub struct FileHandle {
    path: PathBuf,
    file: Rc<File>,
    mode: FileMode,
}

impl PartialOrd for FileHandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl PartialEq for FileHandle {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl AppIO<CCLang> for FileIO {
    fn open(&self, m: &mut Machine<CCLang>) -> io::Result<()> {
        if let Some(Mode(mode)) = m.pop() {
            if let Some(Text(p)) = m.pop() {
                let path = PathBuf::from(p);
                let create = mode.write || mode.append;
                let truncate = mode.write;
                let oo  = OpenOptions::new()
                                      .read(mode.read || mode.plus)
                                      .write(mode.write || mode.plus)
                                      .append(mode.append)
                                      .create(create)
                                      .truncate(truncate)
                                      .open(path.as_path());

                let f = match oo {
                    Ok(f) => f,
                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            format!("failed to open file '{}'",
                                    path.to_str().unwrap())))
                };

                m.push(Handle(Rc::new(FileHandle{ path: path, file: Rc::new(f), mode: mode })));
                return Ok(());
            }
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no file path"));
        }
        return Err(io::Error::new(io::ErrorKind::InvalidData, "no file mode"));
    }

    fn read(&self, m: &mut Machine<CCLang>) -> io::Result<()> {
        if let Some(Index(i)) = m.pop() {
            if let Some(Handle(mut h)) = m.pop() {
                // get the inner type as Any
                let any: &mut dyn Any = Rc::get_mut(&mut h).unwrap();

                // downcast Any to a FileHandle since we know that's what it is
                let fh: &mut FileHandle = any.downcast_mut::<FileHandle>().unwrap();

                // grab the file from the FileHandle
                let file = Rc::get_mut(&mut fh.file).unwrap();

                // read it
                let mut br = match i {
                    _ if i < 0 => {
                        // get the size of the file
                        let meta = fs::metadata(fh.path.as_path())?;
                        BufReader::with_capacity(meta.len() as usize, file)
                    },
                    i => {
                        BufReader::with_capacity(i as usize, file)
                    }
                };
                let buf = br.fill_buf()?;

                // depending on the mode, store binary or text
                if fh.mode.binary {
                    m.push(Binary(Bytes::copy_from_slice(buf)));
                } else {
                    m.push(Text(String::from_utf8_lossy(buf).to_string()));
                }

                // push the read handle back onto the stack
                m.push(Handle(h));
            }
        }
        Ok(())
    }

    fn write(&self, m: &mut Machine<CCLang>) -> io::Result<()> {
        match m.pop() {
            Some(Binary(b)) => {
                if let Some(Handle(mut h)) = m.pop() {
                    // get the inner type as Any
                    let any: &mut dyn Any = Rc::get_mut(&mut h).unwrap();

                    // downcast Any to a FileHandle since we know that's what it is
                    let fh: &mut FileHandle = any.downcast_mut::<FileHandle>().unwrap();

                    if !fh.mode.binary {
                        return Err(io::Error::new(io::ErrorKind::Other, "writing binary to a text file"));
                    }

                    // grab the file from the FileHandle and write
                    let file = Rc::get_mut(&mut fh.file).unwrap();
                    file.write(b.as_ref())?;

                    // push the handle back onto the stack
                    m.push(Handle(h));
                }
                return Ok(())
            },
            Some(Text(s)) => {
                if let Some(Handle(mut h)) = m.pop() {

                    // get the inner type as Any
                    let any: &mut dyn Any = Rc::get_mut(&mut h).unwrap();

                    // downcast Any to a FileHandle since we know that's what it is
                    let fh: &mut FileHandle = any.downcast_mut::<FileHandle>().unwrap();

                    if fh.mode.binary {
                        return Err(io::Error::new(io::ErrorKind::Other, "writing text to a binary file"));
                    }
                    
                    // grab the file from the FileHandle and write
                    let file = Rc::get_mut(&mut fh.file).unwrap();
                    file.write(s.as_ref())?;

                    // push the handle back onto the stack
                    m.push(CCLang::Handle(h));
                }
                return Ok(())
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "no data to write"))
        }
    }

    fn seek(&self, m: &mut Machine<CCLang>) -> io::Result<()> {
        if let Some(Whence(w)) = m.pop() {
            if let Some(Index(i)) = m.pop() {
                if let Some(Handle(mut h)) = m.pop() {
                    // get the inner type as Any
                    let any: &mut dyn Any = Rc::get_mut(&mut h).unwrap();

                    // downcast Any to a FileHandle since we know that's what it is
                    let fh: &mut FileHandle = any.downcast_mut::<FileHandle>().unwrap();

                    // grab the file from the FileHandle and write
                    let file = Rc::get_mut(&mut fh.file).unwrap();

                    match w {
                        gsm::Whence::Start => file.seek(SeekFrom::Start(i as u64))?,
                        gsm::Whence::End => file.seek(SeekFrom::End(i as i64))?,
                        gsm::Whence::Cur => file.seek(SeekFrom::Current(i as i64))?
                    };
                    
                    // push the handle back onto the stack
                    m.push(CCLang::Handle(h));
                }
            }
        }
        Ok(())
    }

    fn close(&self, m: &mut Machine<CCLang>) -> io::Result<()> {
        if let Some(CCLang::Handle(f)) = m.pop() {
            drop(f);
        }
        Ok(())
    }
}


