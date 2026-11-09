use uefi::{
    CStr16, Result, ResultExt, Status, boot,
    proto::media::file::{File, FileAttribute, FileInfo, FileMode::Read, RegularFile},
};

const MAX_FAT32_FILENAME_LENGTH: usize = 255;

#[repr(align(8))]
struct InfoWrapper {
    buffer: [u8; 1024],
}

pub struct FileWrapper {
    file_len: usize,
    file: RegularFile,
}

impl FileWrapper {
    pub fn new(file_name: &str) -> Result<Self> {
        let mut info_wrapper = InfoWrapper { buffer: [0; 1024] };
        let mut name_buffer = [0; MAX_FAT32_FILENAME_LENGTH + 1];

        let handle = boot::image_handle();
        let mut sfs = boot::get_image_file_system(handle)?;

        let mut volume = sfs.open_volume()?;

        let name = CStr16::from_str_with_buf(file_name, &mut name_buffer)
            .map_err(|_| Status::INVALID_PARAMETER)?;

        let mut file = volume
            .open(name, Read, FileAttribute::empty())?
            .into_regular_file()
            .ok_or(Status::UNSUPPORTED)?;
        let file_len = file
            .get_info::<FileInfo>(&mut info_wrapper.buffer)
            .discard_errdata()?
            .file_size() as usize;

        Ok(FileWrapper { file_len, file })
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let read_bytes = self.file.read(&mut *buffer)?;
        if read_bytes != self.file_len {
            return Err(Status::BUFFER_TOO_SMALL.into());
        }

        Ok(read_bytes)
    }
}
