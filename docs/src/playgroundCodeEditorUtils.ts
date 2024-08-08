import ExecutionEnvironment from '@docusaurus/ExecutionEnvironment';

// eslint-disable-next-line @typescript-eslint/no-var-requires
export const JSONEditor = ExecutionEnvironment.canUseDOM ? require('jsoneditor') : null;

export function ajvInitialization() {
  const ajv = JSONEditor.Ajv({
    allErrors: true,
    verbose: true,
    schemaId: 'auto',
    $data: true,
  });

  ajv.addFormat('float', '^-?d+(.d+)?([eE][+-]?d+)?$');
  ajv.addFormat('double', '^-?d+(.d+)?([eE][+-]?d+)?$');
  ajv.addFormat('int32', '^-?d+$');
  ajv.addFormat('uint32', '^d+$');
  ajv.addFormat('uint', '^d+$');

  return ajv;
}
