import React from 'react';
import '@uiw/react-textarea-code-editor/dist.css';
import TextareaCodeEditor from '@uiw/react-textarea-code-editor';

const CodeEditor = ({ code, setCode }: any) => {
  return (
    <div data-color-mode="dark">
      <TextareaCodeEditor
        value={code}
        language="html"
        placeholder="Make Sure you Type in HTML formatted code for custom footer."
        onChange={(e: any) => setCode(e.target.value)}
        padding={15}
        style={{
          fontSize: 14,
          fontFamily:
            'ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace',
        }}
        minHeight={300}
      />
    </div>
  );
};

export default CodeEditor;
