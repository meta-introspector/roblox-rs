<script>
const PI=Math.PI,cos=Math.cos,sin=Math.sin;
const clk=n=>cos(n*PI/3),rot=n=>-cos(n*PI/3),frm=n=>Math.pow(-1,n);
const sup='⁰¹²³⁴⁵⁶';
let step=0,timer=null,anim=null,animT=0;
const DUR=400;
function lerp(a,b,t){return a+(b-a)*t}
function ease(t){return t<.5?2*t*t:1-Math.pow(-2*t+2,2)/2}
function goTo(n){
  n=((n%7)+7)%7;
  if(anim)cancelAnimationFrame(anim);
  let from=step,to=n;animT=0;const s=performance.now();
  function tick(now){
    animT=Math.min((now-s)/DUR,1);const t=ease(animT);
    drawWave(from,to,t);drawPhase(-PI/2+lerp(from,to,t)*PI/3);
    if(animT<1)anim=requestAnimationFrame(tick);
    else{step=to;anim=null}
  }
  anim=requestAnimationFrame(tick);
  document.getElementById('slbl').textContent='O'+sup[to];
  for(let i=0;i<=6;i++)document.getElementById('r'+i).className=i===to?'hi':'';
  document.getElementById('info').innerHTML=
    `<b style="color:#4af">clock</b>=${clk(to).toFixed(4)} <b style="color:#f55">rotor</b>=${rot(to).toFixed(4)} <b style="color:#4c4">frame</b>=${frm(to)}<br>e₄₇∧e₅₉∧e₇₁ ratio=${rot(to).toFixed(4)} = -cos(${to}π/3)`;
}
function nxt(){goTo(step+1)}function prev(){goTo(step-1)}
function rst(){if(timer)clearInterval(timer);timer=null;goTo(0)}
function play(){if(timer){clearInterval(timer);timer=null;return}goTo(0);timer=setInterval(()=>goTo(step+1),900)}
function drawWave(from,to,t){
  const c=document.getElementById('wave'),x=c.getContext('2d'),W=c.width,H=c.height;
  x.clearRect(0,0,W,H);x.fillStyle='#0d0d18';x.fillRect(0,0,W,H);
  const mx=40,my=14,gw=W-2*mx,gh=H-2*my,cy=my+gh/2;
  x.strokeStyle='#1a1a2a';x.lineWidth=1;x.beginPath();x.moveTo(mx,cy);x.lineTo(mx+gw,cy);x.stroke();
  function pl(fn,col,d){
    x.strokeStyle=col;x.lineWidth=2;x.setLineDash(d||[]);x.beginPath();
    for(let n=0;n<=6;n++){const px=mx+n*gw/6,py=cy-fn(n)*gh/2;n?x.lineTo(px,py):x.moveTo(px,py)}
    x.stroke();x.setLineDash([]);x.fillStyle=col;
    for(let n=0;n<=6;n++){const px=mx+n*gw/6,py=cy-fn(n)*gh/2;x.beginPath();x.arc(px,py,n===to?5:2.5,0,2*PI);x.fill()}
  }
  pl(clk,'#4af');pl(rot,'#f55',[5,3]);pl(frm,'#4c4',[2,3]);
  x.strokeStyle='#fa0';x.lineWidth=1;x.setLineDash([6,4]);x.beginPath();x.moveTo(mx,my);x.lineTo(mx+gw,my);x.stroke();x.setLineDash([]);
  const sx=mx+lerp(from,to,ease(t))*gw/6;
  x.strokeStyle='#fff4';x.lineWidth=1.5;x.beginPath();x.moveTo(sx,my);x.lineTo(sx,my+gh);x.stroke();
}
function drawPhase(angle){
  const c=document.getElementById('phase'),x=c.getContext('2d'),W=c.width,H=c.height;
  x.clearRect(0,0,W,H);x.fillStyle='#0d0d18';x.fillRect(0,0,W,H);
  const cx=W/2,cy=H/2,r=100;
  x.strokeStyle='#1a1a2a';x.lineWidth=1;x.beginPath();x.arc(cx,cy,r,0,2*PI);x.stroke();
  for(let i=0;i<6;i++){const a=-PI/2+i*PI/3;x.strokeStyle='#334';x.beginPath();x.moveTo(cx+cos(a)*(r-4),cy+sin(a)*(r-4));x.lineTo(cx+cos(a)*(r+4),cy+sin(a)*(r+4));x.stroke();x.fillStyle='#334';x.font='10px monospace';x.fillText(i,cx+cos(a)*(r+14)-3,cy+sin(a)*(r+14)+4)}
  const ca=angle;
  x.strokeStyle='#4af';x.lineWidth=3;x.beginPath();x.moveTo(cx,cy);x.lineTo(cx+cos(ca)*r*.8,cy+sin(ca)*r*.8);x.stroke();
  x.fillStyle='#4af';x.beginPath();x.arc(cx+cos(ca)*r*.8,cy+sin(ca)*r*.8,5,0,2*PI);x.fill();
  const ra=ca+PI;
  x.strokeStyle='#f55';x.lineWidth=2.5;x.setLineDash([4,3]);x.beginPath();x.moveTo(cx,cy);x.lineTo(cx+cos(ra)*r*.65,cy+sin(ra)*r*.65);x.stroke();x.setLineDash([]);
  x.fillStyle='#f55';x.beginPath();x.arc(cx+cos(ra)*r*.65,cy+sin(ra)*r*.65,4,0,2*PI);x.fill();
  x.fillStyle='#fa0';x.beginPath();x.arc(cx,cy,4,0,2*PI);x.fill();
}
let ganjaAngle=0;
Algebra(3,0,1,()=>{
  const point=(x,y,z)=>1e123+x*1e032+y*1e013+z*1e021;
  const hub=point(0,0,0);
  document.querySelector('.footer').before(this.graph(()=>{
    const t=ganjaAngle,f=cos(t*3)>0?1:-1,fc=f>0?0x44cc44:0xcc4444;
    return[fc,"Frame",point(-1.2,-1.2,0),point(1.2,-1.2,0),point(1.2,1.2,0),point(-1.2,1.2,0),
      0x4488ff,"Clock",[hub,point(.8*cos(-t),.8*sin(-t),0)],point(.8*cos(-t),.8*sin(-t),0),
      0xff5555,"Rotor",[hub,point(.55*cos(-t+PI),.55*sin(-t+PI),0)],point(.55*cos(-t+PI),.55*sin(-t+PI),0),
      0xffaa00,"Hub",hub];
  },{animate:true,grid:true,scale:1,width:'560px',height:'300px'}));
});
goTo(0);
</script>
