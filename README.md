# stash_pd
This program enables you to set an images title, date and Performer purely based on file and path information.

Currently only images are supported.

## Important knowledge
- The Title is just the filename without an Extension. If a title is set, it Stays like you set it.
- Dates are gathered in this Order:
  - From Filename
  - From Title
  - From mtime (Please make sure you don't fuck those up by coping files around)
- Dates must be present in one of the following formats
  - "%Y%m%d"
  - "%Y-%m-%d"
  - "%d%m%Y"
  - "%d-%m-%Y"
- Performers don't get Created, only set. 
- For every image the Complete path gets match against all Performers (Strings will be normalized).

## Usage 

1. Go the stash-app webui and backup your Database!!!
2. Then Export as Json.
3. Now run the tool with --directory_path /path/to/metadata. Do not include images sub-dir!
4. Wait a moment, and after it's done, import the json-data via the webui.
5. Check your existing Data. Make sure everything is correct.
6. Enjoy dates in your Images.
