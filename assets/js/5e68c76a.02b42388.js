"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[247],{936:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>a,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>u,toc:()=>c});var r=n(5893),s=n(1151);const i={},o=void 0,u={id:"api/generated/renderer-OutputStream",title:"renderer-OutputStream",description:"OutputStream",source:"@site/pages/api/generated/renderer-OutputStream.md",sourceDirName:"api/generated",slug:"/api/generated/renderer-OutputStream",permalink:"/docs/api/generated/renderer-OutputStream",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},a={},c=[{value:"OutputStream",id:"outputstream",level:2},{value:"Resolution",id:"resolution",level:2},{value:"Properties",id:"properties",level:4},{value:"EncoderSettings",id:"encodersettings",level:2}];function l(e){const t={code:"code",h2:"h2",h4:"h4",li:"li",pre:"pre",ul:"ul",...(0,s.a)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(t.h2,{id:"outputstream",children:"OutputStream"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type OutputStream = {\n  entity_type: "output_stream",\n  output_id: string,\n  port: u16,\n  ip: string,\n  resolution: Resolution,\n  encoder_settings: EncoderSettings,\n}\n'})}),"\n",(0,r.jsx)(t.h2,{id:"resolution",children:"Resolution"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"type Resolution = {\n  width: u32,\n  height: u32,\n}\n"})}),"\n",(0,r.jsx)(t.h4,{id:"properties",children:"Properties"}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"width"})," - Width in pixels."]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"height"})," - Height in pixels."]}),"\n"]}),"\n",(0,r.jsx)(t.h2,{id:"encodersettings",children:"EncoderSettings"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type EncoderSettings = {\n  preset: \n    | "ultrafast"\n    | "superfast"\n    | "veryfast"\n    | "faster"\n    | "fast"\n    | "medium"\n    | "slow"\n    | "slower"\n    | "veryslow"\n    | "placebo",\n}\n'})})]})}function d(e={}){const{wrapper:t}={...(0,s.a)(),...e.components};return t?(0,r.jsx)(t,{...e,children:(0,r.jsx)(l,{...e})}):l(e)}},1151:(e,t,n)=>{n.d(t,{Z:()=>u,a:()=>o});var r=n(7294);const s={},i=r.createContext(s);function o(e){const t=r.useContext(i);return r.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function u(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:o(e.components),r.createElement(i.Provider,{value:t},e.children)}}}]);