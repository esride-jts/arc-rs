import os
import sys

# Just for integration testing
sys.path.append(os.path.dirname(__file__))

import arcrs

class ToolRegistry(object):
    """
    Manages all available custom Rust tools.
    """

    def __init__(self):
        self._toolbox = arcrs.create_toolbox('Rust Tools', 'rust_tools')
        self._tools = self._toolbox.tools()
    
    def find_tool(self, tool_label):
        """
        Returns the tool having the specified label otherwise None is returned.
        """

        for tool in self._tools:
            if tool_label == tool.label:
                return tool

        return None