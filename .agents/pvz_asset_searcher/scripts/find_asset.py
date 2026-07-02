#!/usr/bin/env python3
import os
import sys
import re

ASSETS_ROOT = "/Users/fregd/Documents/code/playground/PvZ/PvZ_Assets"

STOP_WORDS = {
    # English stop words
    "list", "assets", "related", "to", "and", "or", "in", "folder", "the", "of", "with", "all", "any", "show", "find",
    # Vietnamese stop words
    "liệt", "kê", "các", "asset", "liên", "quan", "đến", "và", "hoặc", "trong", "thư", "mục", "tìm", "kiếm", "cho", "tôi", "hiển", "thị"
}

def extract_keywords(query):
    # Normalize query: lowercase and remove special characters
    normalized = query.lower()
    # Tokenize words
    words = re.findall(r'\w+', normalized)
    # Filter out stop words and single-character noises
    keywords = [w for w in words if w not in STOP_WORDS and len(w) > 1]
    return keywords

def search(query_str):
    if not os.path.exists(ASSETS_ROOT):
        print(f"Error: Assets directory not found at {ASSETS_ROOT}")
        sys.exit(1)
        
    keywords = extract_keywords(query_str)
    
    if not keywords:
        print(f"No search keywords extracted from query: '{query_str}'")
        print("Please provide at least one descriptive keyword (e.g., 'lawn', 'sunflower').")
        sys.exit(1)
        
    print(f"Extracted search keywords: {keywords}")
    print(f"Searching in {ASSETS_ROOT}...")
    print("-" * 80)
    
    matches = []
    
    for root, dirs, files in os.walk(ASSETS_ROOT):
        for name in files:
            rel_path = os.path.relpath(os.path.join(root, name), ASSETS_ROOT)
            rel_path_lower = rel_path.lower()
            
            # Calculate match score: number of keywords matching this file path
            match_score = sum(1 for kw in keywords if kw in rel_path_lower)
            
            if match_score > 0:
                full_path = os.path.join(root, name)
                size_bytes = os.path.getsize(full_path)
                
                # Format size human-readably
                if size_bytes < 1024:
                    size_str = f"{size_bytes} B"
                elif size_bytes < 1024 * 1024:
                    size_str = f"{size_bytes / 1024:.1f} KB"
                else:
                    size_str = f"{size_bytes / (1024 * 1024):.1f} MB"
                    
                matches.append((rel_path, size_str, full_path, match_score))
                
    if not matches:
        print("No matching assets found.")
        return
        
    # Sort matches:
    # 1. Highest match score (descending)
    # 2. Relative path length (ascending, to favor cleaner/shorter names)
    # 3. Relative path (alphabetically)
    matches.sort(key=lambda x: (-x[3], len(x[0]), x[0]))
    
    print(f"Found {len(matches)} matching files (ordered by relevance):")
    for rel, size, full, score in matches:
        relevance_indicator = "⭐" * score
        print(f"- {rel} ({size}) [Match Score: {score} {relevance_indicator}]")
        print(f"  Path: file://{full}")
    print("-" * 80)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 find_asset.py <search_query>")
        sys.exit(1)
    
    # Combine all arguments as the full query string
    full_query = " ".join(sys.argv[1:])
    search(full_query)
