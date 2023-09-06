# Endianness

Endianness, or byte order, is extremely important when working with raw bytes. Attempting to read or
write from a byte source without first ensuring you are reading using the correct byte order will
produce garble or even (probably) **undefined behaviour**.
