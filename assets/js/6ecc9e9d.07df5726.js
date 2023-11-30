"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[646],{6239:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>l,contentTitle:()=>r,default:()=>h,frontMatter:()=>s,metadata:()=>d,toc:()=>c});var t=i(5893),o=i(1151);const s={},r="View",d={id:"api/components/View",title:"View",description:"Properties",source:"@site/pages/api/components/View.md",sourceDirName:"api/components",slug:"/api/components/View",permalink:"/docs/api/components/View",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Tiles",permalink:"/docs/api/components/Tiles"},next:{title:"WebView",permalink:"/docs/api/components/WebView"}},l={},c=[{value:"Properties",id:"properties",level:4},{value:"Transition",id:"transition",level:2}];function a(e){const n={code:"code",h1:"h1",h2:"h2",h4:"h4",li:"li",pre:"pre",ul:"ul",...(0,o.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"view",children:"View"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:'type View = {\n  background_color_rgba?: string,\n  bottom?: f32,\n  children?: Component[],\n  direction?: "row" | "column",\n  height?: f32,\n  id?: string,\n  left?: f32,\n  overflow?: "visible" | "hidden" | "fit",\n  right?: f32,\n  rotation?: f32,\n  top?: f32,\n  transition?: Transition,\n  type: "view",\n  width?: f32,\n}\n'})}),"\n",(0,t.jsx)(n.h4,{id:"properties",children:"Properties"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"background_color_rgba"}),' - (default="#00000000") Background color in a "#RRGGBBAA" format.']}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"bottom"})," - Distance between the bottom edge of this component and the bottom edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"direction"}),' - Direction defines how static children are positioned inside the View component. "row" - Children positioned from left to right. "column" - Children positioned from top to bottom.']}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"height"})," - Height of a component in pixels. Required when using absolute positioning."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"left"})," - Distance between the left edge of this component and the left edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"overflow"}),' - (default="hidden") Controls what happens to content that is too big to fit into an area.']}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"right"})," - Distance between the right edge of this component and the right edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"rotation"})," - Rotation of a component in degrees. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"top"})," - Distance between the top edge of this component and the top edge of its parent. If this field is defined, then component will ignore a layout defined by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"transition"})," - Defines how this component will behave during a scene update. This will only have an effect if previous scene already contained a View component with the same id."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"width"})," - Width of a component in pixels. Required when using absolute positioning."]}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"transition",children:"Transition"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:"type Transition = {\n  duration_ms: f64,\n}\n"})})]})}function h(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(a,{...e})}):a(e)}},1151:(e,n,i)=>{i.d(n,{Z:()=>d,a:()=>r});var t=i(7294);const o={},s=t.createContext(o);function r(e){const n=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:r(e.components),t.createElement(s.Provider,{value:n},e.children)}}}]);