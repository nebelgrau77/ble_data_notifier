MEMORY
{
  /* s140 7.3.0 */
  FLASH : ORIGIN = 0x00027000, LENGTH = 0x7F000 - 0x27000 /* save 4K for flash storage */
  RAM : ORIGIN = 0x2000DA50, LENGTH = 0x20010000 - 0x2000DA50
}