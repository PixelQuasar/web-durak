const n=function(){return`\n<div>\n    <h1>\n        "${this.notFoundPage??"unknown"}" instance is not found.\n    </h1>\n</div>\n    `};(function(e){const t=[];try{for(const n of e)n.name&&n.page&&t.push(n)}catch(n){console.error("Error: unable to build router: ",n)}return{render:function(){let e=t.map((n=>{return{route:n,result:location.pathname.match((e=n.name,new RegExp("^"+e.replace(/\//g,"\\/").replace(/:\w+/g,"(.+)")+"$")))};var e})).find((n=>null!==n.result));e||(e={route:{name:"404",page:n.bind({notFoundPage:location.pathname})},result:[location.pathname]});document.querySelector("#root").innerHTML=e.route.page.call(function(n){const e=n.result.slice(1),t=Array.from(n.route.name.matchAll(/:(\w+)/g)).map((n=>n[1]));return Object.fromEntries(t.map(((n,t)=>[n,e[t]])))}(e))}}})([{name:"/",page:function(){return"\n<div>\n    Homepage\n</div>\n    "}},{name:"/join",page:function(){return"\n<div>\n    Join lobby page\n</div>\n    "}},{name:"/create",page:function(){return"\n<div>\n    Create lobby page\n</div>\n    "}}]).render();
