<h1> Lossy PPM Image Compressor </h1>

<h2> Description </h2>
This project is a lossy image compressor and decompressor that converts between full-color portable pixmap images and compressed binary image files. This program works on 2x2 blocks of pixels and resulting the compressed image is about three times smaller than the origional. The steps of the program are as follows:

- Reads a PPM image from a file specified on the command line or from standard input and trims the rows and/or columns, if necessary, to make the height and width of the image even numbers
- Changes the pixel RGB values to a floating-point representation
- Transforms each pixel from RGB color space into component video color space (Y / P<sub>B</sub> / P<sub>R</sub>)
- Pack each 2-by-2 block into a 32-bit word as follows:
  - For the P<sub>B</sub> and P<sub>R</sub> (chroma) elements of the pixels, averages the values of the four pixels in the block.
  - Convert the PB and PR elements to four-bit values
  - Uses a discrete cosine transform (DCT), to transform the four Y (luminance/luma) values of the pixels into cosine coeffecients a, b, c, and d.
  - Convert the b, c, and d to five-bit signed values assuming that they lie between −0.3 and 0.3 and if not clamps them to −0.3 and 0.3.
- Writes the compressed binary image (sequence of 32-bit codewords) to standard output

<h2> Languages Used </h2>

- <b> Rust </b>

<h2> Usage: </h2>
Takes the option -c (for compress) or -d (for decompress) and also the name of the file to compress or decompress. The name of the file may be omitted, in which case standard input is used.
