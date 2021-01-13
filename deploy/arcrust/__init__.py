import os
import sys

# Just for integration testing
sys.path.append(os.path.dirname(__file__))

# Import the native Python modules
import copyfeatures

class Tool(object):
    """
    Represents a simple wrapper for a Rust tool.
    """

    def __init__(self, toolbox, tool_index):
        self._toolbox = toolbox
        self._tool_index = tool_index

    @property
    def label(self):
        """
        The label of this tool.
        """
        return self._toolbox.tool_label(self._tool_index)

    @property
    def description(self):
        """
        The description of this tool.
        """
        return self._toolbox.tool_description(self._tool_index)

    def parameter_info(self):
        """
        Returns the parameter infos of this tool.
        """
        return self._toolbox.tool_parameter_info(self._tool_index)

    def execute(self, parameters, messages):
        """
        Executes this tool.
        """
        self._toolbox.tool_execute(self._tool_index, parameters, messages)



class ToolRegistry(object):
    """
    Manages all available custom Rust tools.
    """

    def __init__(self):
        self._tool_labels = []
        self._tools = []
        copy_toolbox = copyfeatures.create_toolbox('Rust Copy Tools', 'rust_copy_tools')
        self._tool_labels.extend(copy_toolbox.tools())
        self._tools.extend([Tool(copy_toolbox, tool_index) for tool_index in range(0, len(self._tool_labels))])
    
    def find_tool(self, tool_label):
        """
        Returns the tool having the specified label otherwise None is returned.
        """
        for tool_index in range(0, len(self._tool_labels)):
            known_label = self._tool_labels[tool_index]
            if tool_label == known_label:
                return self._tools[tool_index]

        return None

    def list_tools(self):
        """
        Returns all tool names/labels being registered.
        """
        return self._tool_labels
        