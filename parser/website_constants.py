from enum import Enum
import os

class Paths(Enum):
    REPO_BASE_DIR = os.getcwd().replace('/parser', '')