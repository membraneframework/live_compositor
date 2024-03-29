import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  sidebar: [
    'intro',
    {
      label: 'Get started',
      type: 'category',
      items: ['get-started/elixir', 'get-started/node'],
      collapsed: true,
      link: {
        type: 'doc',
        id: 'get-started',
      },
    },
    {
      type: 'category',
      label: 'Concepts',
      collapsible: false,
      items: ['concept/component', 'concept/layouts', 'concept/shaders', 'concept/web'],
    },
    {
      type: 'category',
      label: 'Deployment',
      collapsible: true,
      link: {
        type: 'generated-index',
      },
      items: [
        {
          type: 'doc',
          id: 'deployment/configuration',
          label: 'Configuration',
        },
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      collapsible: false,
      link: {
        type: 'generated-index',
      },
      items: [
        {
          type: 'doc',
          id: 'api/routes',
          label: 'HTTP Routes',
        },
        {
          type: 'doc',
          id: 'api/events',
          label: 'Events',
        },
        {
          type: 'category',
          label: 'Components',
          collapsible: false,
          description: 'Basic blocks used to define a scene.',
          items: [
            {
              type: 'autogenerated',
              dirName: 'api/components',
            },
          ],
        },
        {
          type: 'category',
          label: 'Renderers',
          collapsible: false,
          description: 'Resources that need to be registered first before they can be used.',
          items: ['api/renderers/shader', 'api/renderers/image', 'api/renderers/web'],
        },
        {
          type: 'category',
          label: 'Outputs',
          collapsible: false,
          description: 'Elements that deliver generated media.',
          items: ['api/outputs/rtp'],
        },
        {
          type: 'category',
          label: 'Inputs',
          collapsible: false,
          description: 'Elements that deliver media from external sources.',
          items: ['api/inputs/rtp', 'api/inputs/mp4'],
        },
      ],
    },
  ],
};

export default sidebars;
