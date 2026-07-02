import struct
import os

pak_path = "/Applications/Plants vs. Zombies.app/Contents/Resources/Plants vs. Zombies.app/Contents/Resources/main.pak"
output_dir = "/Users/fregd/Documents/code/playground/PvZ/extracted_assets"

def extract():
    if not os.path.exists(pak_path):
        print(f"Error: {pak_path} does not exist.")
        return

    print("Reading and decrypting main.pak...")
    with open(pak_path, "rb") as f:
        raw_data = f.read()
    
    # Decrypt in-place using XOR with 0xf7
    decrypted = bytearray(raw_data)
    for i in range(len(decrypted)):
        decrypted[i] ^= 0xf7
        
    print("Parsing metadata...")
    offset = 8
    files = []
    while offset < len(decrypted):
        flags = decrypted[offset]
        if flags & 0x80:
            offset += 1
            break
        
        fnamesz = decrypted[offset + 1]
        fname_bytes = decrypted[offset + 2 : offset + 2 + fnamesz]
        # Replace backslashes with forward slashes for cross-platform compatibility
        fname = fname_bytes.decode('utf-8', errors='replace').replace('\\', '/')
        
        size_offset = offset + 2 + fnamesz
        size = struct.unpack("<I", decrypted[size_offset : size_offset + 4])[0]
        tstamp = struct.unpack("<Q", decrypted[size_offset + 4 : size_offset + 12])[0]
        
        files.append({
            'name': fname,
            'size': size,
            'tstamp': tstamp
        })
        offset = size_offset + 12
        
    data_offset = offset
    print(f"Metadata parsed successfully. Found {len(files)} files.")
    print(f"Data section starts at offset: {data_offset}")
    
    # Create the output directory
    os.makedirs(output_dir, exist_ok=True)
    
    extracted_count = 0
    
    current_data_offset = data_offset
    for f_info in files:
        fname = f_info['name']
        size = f_info['size']
        
        # Get raw data
        file_data = decrypted[current_data_offset : current_data_offset + size]
        current_data_offset += size
        
        # Determine target file path
        target_path = os.path.join(output_dir, fname)
        target_dir = os.path.dirname(target_path)
        os.makedirs(target_dir, exist_ok=True)
        
        # Write file data exactly as-is (no slicing)
        with open(target_path, "wb") as out_f:
            out_f.write(file_data)
            
        extracted_count += 1
        if extracted_count % 500 == 0:
            print(f"Extracted {extracted_count}/{len(files)} files...")
            
    print(f"\nExtraction complete!")
    print(f"Successfully extracted {extracted_count} files to {output_dir}")

if __name__ == "__main__":
    extract()
