use std::fs::File;
use std::io::Write;




//#![allow(dead_code)]
const luma_width: i32=128;
const luma_height: i32=96;
const chroma_width: i32=luma_width / 2;
const chroma_height: i32=luma_height / 2;

// H.264 bitstreams
const sps: [u8; 11] = [ 0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00, 0x0a, 0xf8, 0x41, 0xa2 ];
const pps: [u8; 8] = [ 0x00, 0x00, 0x00, 0x01, 0x68, 0xce, 0x38, 0x80 ];
const slice_header: [u8; 9] = [ 0x00, 0x00, 0x00, 0x01, 0x05, 0x88, 0x84, 0x21, 0xa0 ];
const macroblock_header: [u8; 2] = [ 0x0d, 0x00 ];


// Write a macroblock's worth of YUV data in I_PCM mode

pub fn macroblock(i: i32, j: i32) {

  if i != 0 && j != 0 {
    println!("you could type that too if ou want i think umm close double nooo he he");
    // fwrite(&macroblock_header, 1, sizeof(macroblock_header), stdout);
  }

  for x in (i * 16)..((i + 1) * 16) {
      for y in (j * 16)..((j + 1) * 16) {
        println!("it's important");
        //   fwrite(&frame.Y[x][y], 1, 1, stdout);
      }
  }
  for x in (i * 8)..((i + 1) * 8) {
      for y in (j * 8)..((j + 1) * 8) {
        println!("it's important");
        //   fwrite(&frame.Y[x][y], 1, 1, stdout);
      }
  }
  for x in (i * 8)..((i + 1) * 8) {
      for y in (j * 8)..((j + 1) * 8) {
        println!("it's important");
        //   fwrite(&frame.Y[x][y], 1, 1, stdout);
      }
  }
  // for(x = i*16; x < (i+1)*16; x++)
  //   for (y = j*16; y < (j+1)*16; y++)
  //     fwrite(&frame.Y[x][y], 1, 1, stdout);


  // for (x = i*8; x < (i+1)*8; x++)
  //   for (y = j*8; y < (j+1)*8; y++)
  //     fwrite(&frame.Cb[x][y], 1, 1, stdout);


  // for (x = i*8; x < (i+1)*8; x++)
  //   for (y = j*8; y < (j+1)*8; y++)
  //     fwrite(&frame.Cr[x][y], 1, 1, stdout);
}

/* Write out PPS, SPS, and loop over input, writing out I slices */
pub fn main() {

    let data = "Some data!";
    let mut f = File::create("foo").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");


      for i in 0..(luma_height/16) {
        for j in 0..(luma_width/16) {
            macroblock(i, j);
        }
    }




  // fwrite(sps, 1, sizeof(sps), stdout);
  // fwrite(pps, 1, sizeof(pps), stdout);

//   rory: no idea what "feof" is
//   while (! feof(stdin))
//   {
//     fread(&frame, 1, sizeof(frame), stdin);
//     fwrite(slice_header, 1, sizeof(slice_header), stdout);


    // for (i = 0; i < luma_height/16 ; i++)
    //   for (j = 0; j < luma_width/16; j++)
    //     macroblock(i, j);

//     fputc(0x80, stdout); /* slice stop bit */
//   }

}