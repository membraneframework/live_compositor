import React, { useState, useEffect, useCallback } from 'react';
import 'jsoneditor/dist/jsoneditor.css';
import './jsoneditor-dark.css';
import componentTypesJsonSchema from '../../../schemas/component_types.schema.json';
import { ajvInitialization, JSONEditor } from '../playgroundCodeEditorUtils';

interface PlaygroundCodeEditorProps {
  onChange: (content: object | Error) => void;
  initialCodeEditorContent: object;
}

function PlaygroundCodeEditor({ onChange, initialCodeEditorContent }: PlaygroundCodeEditorProps) {
  const [jsonEditor, setJsonEditor] = useState<typeof JSONEditor | null>(null);

  const editorContainer = useCallback(node => {
    if (node === null) {
      return;
    }
    const ajv = ajvInitialization();
    const validate = ajv.compile(componentTypesJsonSchema);

    const editor = new JSONEditor(node, {
      mode: 'code',
      enableSort: false,
      enableTransform: false,
      statusBar: false,
      mainMenuBar: false,
      ajv,
      onChange: () => {
        try {
          const jsonContent = editor.get();
          onChange(jsonContent);
          if (!validate(jsonContent)) throw new Error('Invalid JSON!');
        } catch (error) {
          onChange(error);
        }
      },
    });

    editor.setSchema(componentTypesJsonSchema);
    editor.set(initialCodeEditorContent);

    setJsonEditor(editor);
  }, []);

  useEffect(() => {
    return () => {
      if (jsonEditor) {
        jsonEditor.destroy();
      }
    };
  }, [jsonEditor]);

  return <div ref={editorContainer} style={{ height: '100%' }} />;
}

export default PlaygroundCodeEditor;
