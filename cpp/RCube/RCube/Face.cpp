#include "Face.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string>

Face::Face() {
  data = nullptr;
  RowSize = 0;
  MemRowSize = 0;
  BS = 0;
  DataSize = 0;
  orientation = 0;
  RotatefaceCW(0);
}

void Face::Initialize(byte index, uint rsize, uint msize) {

  // if(rsize > 1024) printf("Initializing Face %i\n", (int)index);

  // Index of this face
  id = index;

  // Length of a row of memory  (must be a power of 2)
  MemRowSize = msize;

  // Length of a side of the cube
  RowSize = rsize;
  R1 = RowSize - 1;

  // Compute the bit-shift size for the length of a row
  BS = (int)log2(MemRowSize);

  // Compute the actual size of the array in bytes
  DataSize = ((uint64)MemRowSize) * ((uint64)MemRowSize);
  std::cout << "DataSize: " << DataSize << std::endl;

  // Initialize array
  data = new byte[DataSize];
  std::cout << "Data: " << data << std::endl;

  // Failed to allocate array
  if (data == nullptr) {
    printf("ERROR ALLOCATING ARRAY");
  }

  // Set all values on this face to match the faceindex
  PaintPictureCube(index);
}

// Verify the number of pieces on this face match what was saved and loaded from
// file
bool Face::VerifyCounts() {
  uint Counts[6];
  GetCounts(Counts);

  for (int i = 0; i < 6; ++i) {
    printf("Face %i : Color %i : %u %u\n", id, i, Counts[i], PieceCount[i]);
    if (Counts[i] != PieceCount[i])
      return false;
  }

  return true;
}

void Face::SaveFaceState() {
  printf("Saving face %i\n", id);

  // GetCounts(PieceCount);
  std::string name = "face" + std::to_string(id) + ".bin";
  std::ofstream out(name, std::ios::out | std::ios::binary);
  out.write((char *)this, sizeof(Face));
  out.write((char *)data, DataSize);
  out.flush();
  out.close();
}

void Face::LoadFaceState(byte faceid) {
  printf("Loading face %i\n", faceid);
  std::string name = "face" + std::to_string(faceid) + ".bin";
  std::ifstream in(name, std::ios::in | std::ios::binary);
  in.read((char *)this, sizeof(Face));
  data = new byte[DataSize];
  in.read((char *)data, DataSize);
  in.close();
}

void Face::PaintPictureCube(byte index) {
  // paint cube with indices from index ** 2 to (index + 1) ** 2 - 1
  if (index == 5) {
    index = 0;
  } else {
    index++;
  }
  int already_painted = index * index;
  int to_paint = already_painted + DataSize;
  for (int i = already_painted; i < to_paint; i++) {
    data[i - already_painted] = i;
  }
}

// Paint the entire face this color
void Face::Paint(byte color) {
  // data is actually an int array, set all int values to color
  for (uint64 i = 0; i < DataSize; ++i) {
    data[i] = color;
  }
}

// Counts the number of pieces on a face of a spefic color
uint Face::Count(byte color) {
  uint result = 0;
  for (uint r = 0; r < RowSize; ++r)
    for (uint c = 0; c < RowSize; ++c)
      if (GetRC(r, c) == color)
        ++result;

  return result;
}

// Records the number of each piece on this face
void Face::GetCounts(uint *cnt) {
  for (int i = 0; i < 6; ++i)
    cnt[i] = 0;

  for (uint r = 0; r < RowSize; ++r) {
    for (uint c = 0; c < RowSize; ++c) {
      cnt[GetRC(r, c)]++;
    }
  }
}

// Returns true if all pieces on this face match the face id (solved state)
bool Face::IsFaceSolved() {
  for (uint r = 0; r < RowSize; ++r) {
    for (uint c = 0; c < RowSize; ++c) {
      if (GetRC(r, c) != this->id)
        return false;
    }
  }

  return true;
}

Face::~Face() {
  // Cleanup
  if (data != nullptr)
    delete[] data;
}
