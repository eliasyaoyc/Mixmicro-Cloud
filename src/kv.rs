pub trait EsKv {
    fn write();
    fn batch_write();
    fn read();
    fn batch_read();
    fn get();
    fn list();
}