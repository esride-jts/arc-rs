import arcpy
import arcrust

tool_registry = arcrust.ToolRegistry()

class Toolbox(object):
    def __init__(self):
        self.label =  'Arc Rust Integration toolbox'
        self.alias  = 'arcrust_integration'
        self.tools = [TestTool]
        


class TestTool(object):
    """
    Simple Python tool wrapping an existing Rust tool.
    """
    
    def __init__(self):
        self._rust_tool = tool_registry.find_tool('Dummy Tool')
        if None is self._rust_tool:
            raise ValueError('Rust tool is not registered!')

        self.label = self._rust_tool.label
        self.description = self._rust_tool.description

    def getParameterInfo(self):
        return self._rust_tool.parameter_info()

    def isLicensed(self): #optional
        return True

    def updateParameters(self, parameters): #optional
        return

    def updateMessages(self, parameters): #optional
        return

    def execute(self, parameters, messages):
        self._rust_tool.execute(parameters, messages)
