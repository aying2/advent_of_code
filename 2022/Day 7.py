import sys
import re

class Folder:
    def __init__(self, name, parent, children, size):
        self.name = name
        self.parent = parent
        self.children = children
        self.size = size
    def totalSize(self):
        total_size = self.size
        for child in self.children:
            total_size += child.totalSize()
        return total_size
    
class Filesystem:
    def __init__(self, files, current):
        self.files = files
        self.current = current
    def sum_below_threshold(self, threshold = 100000):
        sum = 0
        for folder in self.files:
            if folder.totalSize() <= threshold:
                sum += folder.totalSize()
        return sum
    def find_smallest_delete(self, total_available_size = 70000000, space_needed = 30000000):
        # myFilesystem.files[0].totalSize() should be the home directory
        unused = total_available_size - myFilesystem.files[0].totalSize()
        threshold = space_needed-unused
        min = sys.maxsize
        for folder in self.files:
            if folder.totalSize() >= threshold and folder.totalSize() < min:
                min = folder.totalSize()
        return min

def build_filesystem(filename):
    with open(filename) as ifile:

        myFilesystem = Filesystem([], "")
        
        for line in ifile:
            #identify user commands
            idx_cd = line.find("$ cd")
            idx_ls = line.find("$ ls")

            # handle cd command
            if(idx_cd != -1):
                dir_name = line[idx_cd+5:len(line)-1]
                if(dir_name == "/"):
                    has_home = False

                    # can only have one home folder at top
                    for folder in myFilesystem.files:
                        if (folder.name=="/"):
                            has_home = True
                            # set current to existing home
                            myFilesystem.current = folder
                    if not has_home:
                        myFolder = Folder("/", "/", [], 0)

                        myFilesystem.files.append(myFolder)

                        # set current to new home
                        myFilesystem.current = myFolder
                    
                    
                elif(dir_name == ".."):
                    # move up a level
                    myFilesystem.current = myFilesystem.current.parent
                else:
                    # create empty folder with path information
                    myFolder = Folder(dir_name, myFilesystem.current, [], 0)
                    
                    # if folder is already a child, descend into it
                    # and do not add a new folder
                    already_added = False
                    for folder in myFilesystem.current.children:
                        if (folder.name==myFolder.name):
                            already_added = True

                            myFilesystem.current = folder
                    
                    # add new folder as child, add to general filesystem
                    # descend into new folder
                    if not already_added:
                        myFilesystem.current.children.append(myFolder)
                        myFilesystem.files.append(myFolder)
                        myFilesystem.current = myFolder

            
            elif(idx_ls != -1):
                # go ahead and wipe because it'll be filled in
                myFilesystem.current.size = 0
                
            # not a user command
            else:
                idx_dir = line.find("dir")
                if(idx_dir != -1):
                    # similar to above logic, but no descent
                    dir_name = line[idx_dir+4:len(line)-1]

                    myFolder = Folder(dir_name, myFilesystem.current, [], 0)

                    already_added = False
                    for folder in myFilesystem.current.children:
                        if (folder.name==myFolder.name):
                            already_added = True
                    
                    if not already_added:
                        myFilesystem.current.children.append(myFolder)
                        myFilesystem.files.append(myFolder)
                else:
                    size = re.findall('\d+', line)
                    if(size==[]):
                        size = 0
                    else:
                        size = eval(size[0])
                    myFilesystem.current.size += size
        return myFilesystem


filename = sys.argv[1]
myFilesystem = build_filesystem(filename)

# for folder in myFilesystem.files:
#     print(folder.name)
#     print(folder.totalSize())
#     for folder in folder.children:
#         print("child", folder.name)

# print(myFilesystem.sum_below_threshold())

# for folder in myFilesystem.files:
#     if folder.name == '/':
#         for child in folder.children:
#             print(child.name, child.totalSize())

print(myFilesystem.find_smallest_delete())


