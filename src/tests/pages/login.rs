pub const LOGIN_PAGE: &str = r##"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN"
"http://www.w3.org/TR/html4/loose.dtd">
<HTML>
  <HEAD>
		<title>浙江大学现代教务管理系统</title>
		<meta http-equiv="Content-Type" content="text/html; charset=gb2312">
		<meta http-equiv="Content-Language" content="gb2312">
		<meta content="all" name="robots">
		<meta name="author" content="">
		<meta name="Copyright" content="浙江大学">
		<meta name="description" content="浙江大学">
		<link rel="stylesheet" rev="stylesheet" href="style/base.css" type="text/css" media="all">
			<link rel="stylesheet" rev="stylesheet" href="style/form.css" type="text/css" media="all">
				<link rel="stylesheet" rev="stylesheet" href="style/module.css" type="text/css" media="all">
	    <script src="/ajax/common.ashx" type="text/javascript"></script>
		<script src="/ajax/zjdx.AjaxForm,zjdx.ashx" type="text/javascript"></script>
					<script language="javascript">
				 function Chose(obj){
				      var str=obj.id;
				      //alert(str);
				      if (str=="gg"){
				          document.getElementById("tzgg").className="current";
				         document.getElementById("tzgg1").className="";
				         document.getElementById("Db_gg1").style.display="none";
				         document.getElementById("Db_gg").style.display="block";
				      }
				      else
				      {
				         document.getElementById("tzgg").className="";
				         document.getElementById("tzgg1").className="current";
				         document.getElementById("Db_gg1").style.display="block";
				         document.getElementById("Db_gg").style.display="none";
				      }

				 }
				 function pdxmmgs(){
				      var xh=document.getElementById("TextBox1");
				      var mm=document.getElementById("TextBox2");
				      var lx=document.getElementById("RadioButtonList1");
				      var radio= lx.getElementsByTagName("INPUT");
				      var result;
				      for(var i=0;i<radio.length;i++)
                      {
                          if(radio[i].checked)
                          {
                             result=radio[i].value;
                          }
                      }
				      var fhjg=AjaxForm.Pdmm(xh.value,mm.value,result);
				      if(fhjg.value=="1")
				      {
								var str=mm.value;
								if (str!="")
								{
									var pdjg=pd(str);

									if (pdjg==true)
									{
									alert("您的密码过于简单，存在安全隐患，请及时修改！！");
									}
								}
					   }
				 }

				 function pd(obj)
				 {
				                    var Reg1=/^[0-9]*$/g
									var Reg2=/^[A-Za-z]*$/g
									var r1=obj.match(Reg1)
									var r2=obj.match(Reg2)
									if (obj.length<=6)
									{
										return true;
									}
									if(r1!=null)
									{
										return true;
									}
									if(r2!=null)
									{
										return true;
									}
									return false;
				 }
					</script>
					<style>
					#RadioButtonList1 TD { PADDING-RIGHT: 0px; PADDING-LEFT: 0px; PADDING-BOTTOM: 2px; PADDING-TOP: 2px ;}
					.width100{width:100px;}
					.width50{width:50px;}
					.tabpadding,.tabpadding td{padding:0;margin:0;border-collapse: collapse;}
					.tabpadding input{padding:1px;margin:1px}
					a img{ border:none; }
					</style>
</HEAD>
	<body>
		<form name="Form1" method="post" action="default2.aspx" id="Form1">
<input type="hidden" name="__EVENTTARGET" value="" />
<input type="hidden" name="__EVENTARGUMENT" value="" />
<input type="hidden" name="__VIEWSTATE" value="dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=" />

<script language="javascript" type="text/javascript">
<!--
	function __doPostBack(eventTarget, eventArgument) {
		var theform;
		if (window.navigator.appName.toLowerCase().indexOf("microsoft") > -1) {
			theform = document.Form1;
		}
		else {
			theform = document.forms["Form1"];
		}
		theform.__EVENTTARGET.value = eventTarget.split("$").join(":");
		theform.__EVENTARGUMENT.value = eventArgument;
		theform.submit();
	}
// -->
</script>

			<div class="indextop"></div>
			<div class="indexmid">
				<div class="toplogo">
					<h6></h6>
				</div>
				<div class="imglogin">
					<h6></h6>
					<h5>
						<div class="title"><FONT face="宋体"></FONT><FONT face="宋体"></FONT><FONT face="宋体"></FONT><a href="https://zjuam.zju.edu.cn:8443/amserver/UI/Login?goto=http://jwbinfosys.zju.edu.cn" target="_self"></a></div>
						<table align="center" width="200">
							<tr>
								<td width="50"><span id="Label1">用户名：</span></td>
								<td><input name="TextBox1" type="text" id="TextBox1" tabindex="1" class="inputtext width100" /></td>
							</tr>
							<tr>
								<td><span id="Label2">密 &nbsp; 码：</span></td>
								<td><input name="TextBox2" type="password" id="TextBox2" tabindex="2" class="inputtext width100" /></td>
							</tr>
							<tr>
								<td><span id="Label4">验证码：</span></td>
								<td><input name="Textbox3" type="text" maxlength="5" id="Textbox3" tabindex="2" class="inputtext width50" style="FLOAT:left" /><IMG src="CheckCode.aspx" height="24" style="FLOAT:left"></td>
							</tr>
							<tr>
								<td colspan="2">
									<table id="RadioButtonList1" class="tabpadding" border="0" width="100%">
	<tr>
		<td><input id="RadioButtonList1_0" type="radio" name="RadioButtonList1" value="部门" tabindex="3" /><label for="RadioButtonList1_0">部门　</label></td><td><input id="RadioButtonList1_1" type="radio" name="RadioButtonList1" value="教师" tabindex="3" /><label for="RadioButtonList1_1">教师　</label></td><td><input id="RadioButtonList1_2" type="radio" name="RadioButtonList1" value="学生" checked="checked" tabindex="3" /><label for="RadioButtonList1_2">学生　</label></td><td><input id="RadioButtonList1_3" type="radio" name="RadioButtonList1" value="访客" tabindex="3" /><label for="RadioButtonList1_3">访客</label></td>
	</tr>
</table></td>
							</tr>
							<tr>
								<td colspan="2">
									<a id="Button1" class="dl" onclick="pdxmmgs();" href="javascript:__doPostBack('Button1','')"></a><a href="yjs_xxqr.aspx">研究生首次登陆</a>
								</td>
							</tr>
						</table>
					</h5>
				</div>
				<!--开始-->
				<div class="de_main">
					<div class="de_left">
						<!--新闻列表-->
						<div class="de_news">
							<h3><span>本科生院通知公告</span><a href="jwggcx.aspx?type=1" target="_blank">【更多】</a></h3>
							<div class="con">
								<table class="datagridstyle" cellspacing="0" cellpadding="3" border="0" id="DataGrid1" width="100%">
	<tr class="datagridhead">
		<td>公告标题</td><td>发布单位</td><td>发布时间</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于我校全日制普通本科学生信息采集的通知.doc','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于我校全日制普通本科学生信息采集的通知</a></td><td>本科生院</td><td>2019-01-28 12:04:17</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/3 关于本科生2018-2019学年春夏学期报到注册的温馨提示20190115.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于本科生2018-2019学年春夏学期报到注册的温馨提示</a></td><td>本科生院</td><td>2019-01-28 12:03:19</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于删除《马克思主义基本原理概论》部分2018级学生的通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于删除2018-2019春夏学期《马克思主义基本原理概论》部分2018级学生的通知</a></td><td>课程中心</td><td>2019-01-24 08:35:33</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/附件1：关于2017级、2018级学生本学期转专业有关事项的通知(1).docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2017级、2018级学生春夏学期转专业有关事项的通知</a></td><td>本科生院</td><td>2019-01-18 10:07:59</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/2018-2019学年春学期初部分统一补考课程时间安排表.xlsx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  【查询】2018-2019学年春学期初部分统一补考课程时间安排表</a></td><td>课程中心</td><td>2019-01-15 11:46:09</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于2018-2019学年春夏学期本科课程第二轮选课情况的通报.pdf','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2018-2019学年春夏学期本科课程第二轮选课情况的通报</a></td><td>课程中心</td><td>2019-01-08 17:37:04</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于2018-2019学年春夏学期本科课程第一轮选课情况的通报.pdf','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2018-2019学年春夏学期本科课程第一轮选课情况的通报</a></td><td>课程中心</td><td>2019-01-03 17:02:48</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于2017级及以前学生修读通识选修课程的补充说明 (含“沟通与领导类”课程清单).docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2017级及以前学生修读通识选修课程的补充说明 (含“沟通与领导类”课程清单)</a></td><td>通识教育中心</td><td>2018-12-26 15:01:43</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://bksy.zju.edu.cn/office/redir.php?catalog_id=711393&object_id=1180253','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于公布2017级创新创业类课程群（第一批）的通知</a></td><td>通识中心</td><td>2018-12-25 17:01:14</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于2018-2019学年春夏学期本科课程选课安排的通知3.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  【重要】关于2018-2019学年春夏学期本科课程选课安排的通知</a></td><td>课程中心</td><td>2018-12-23 12:12:09</td>
	</tr>
</table>
							</div>
						</div>
						<!--新闻列表-->
						<div class="de_news">
							<h3><span>各院系通知公告</span><a href="jwggcx.aspx?type=2" target="_blank">【更多】</a></h3>
							<div class="con">
								<table class="datagridstyle" cellspacing="0" cellpadding="3" border="0" id="Datagrid2" width="100%">
	<tr class="datagridhead">
		<td>公告标题</td><td>发布单位</td><td>发布时间</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于2019年春夏学期申请转入生物科学（求是科学班）的通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2019年春夏学期申请转入生物科学（求是科学班）的通知</a></td><td>生科院</td><td>2019-01-28 21:45:25</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/马克思主义学院关于2018-2019春夏学期夏婉婉等教师《马克思主义基本原理概论》课程更换任课教师通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  马克思主义学院关于2018-2019春夏学期夏婉婉等教师《马克思主义基本原理概论》课程更换任课教师通知</a></td><td>马克思主义学院</td><td>2019-01-28 11:39:37</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/管理学院关于2018-2019春夏学期停课通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  管理学院关于2018-2019春夏学期课程的停课通知</a></td><td>管理学院</td><td>2019-01-28 11:00:35</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://www.chem.zju.edu.cn/redir.php?catalog_id=437&object_id=123007','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  关于2019年春夏学期申请转入化学求是科学班的通知</a></td><td>化学系</td><td>2019-01-22 15:31:05</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/马克思主义学院关于2018-2019学年春夏学期马克思主义基本原理概论课程金建伟老师等六位老师的六个班级停班的通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  马克思主义学院2018-2019学年春夏学期“马克思主义基本原理概论”课程金建伟等六位老师共六个班级停班通知</a></td><td>马克思主义学院</td><td>2019-01-18 16:29:45</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/2019-1-15 外语学院关于2018-2019春夏学期大学英语等F、R类课程部分教学班停开的通知.doc','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  外语学院关于2018-2019学年春夏学期《大学英语Ⅳ》等F/R类课程部分教学班停开的通知</a></td><td>外国语言文化与国际交流学院</td><td>2019-01-17 19:22:45</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/物理系关于选拔2017级、2018级优秀学生进入求是科学班学习的通知.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  物理系关于选拔2017级、2018级优秀学生进入求是科学班学习的通知</a></td><td>物理学系</td><td>2019-01-10 11:02:57</td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/2018-2019学年秋冬学期大学物理课程答疑安排.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')"><img src=images/tzgg_icon.gif border=0/>  2018-2019学年秋冬学期大学物理课程答疑安排</a></td><td>物理学系</td><td>2018-10-18 17:16:37</td>
	</tr>
</table>
							</div>
						</div>
					</div>
					<div class="de_right">
						<a href="http://bksy.zju.edu.cn/dwjlfwpt/" target=_blank><IMG src="images/piclink_dwjl.jpg" border="0"></a>
						<a href="http://10.202.78.14/jwglxt/" target=_blank><IMG src="images/piclink_xxfw.jpg" border="0"></a>
						<a href="http://10.202.78.14:8888/sxglxt/" target=_blank><IMG src="images/piclink_xxxt.jpg" border="0"></a>

						<!--下载-->
						<a href="http://bksy.zju.edu.cn/office/redir.php?catalog_id=711183"  target="_blank"><IMG src="images/piclink_pyfa.jpg" border="0"></a>
						<div class="de_down">
							<h3><span>文件下载</span><a href="jwggcx.aspx?type=3" target="_blank">【更多】</a></h3>
							<div class="con">
								<table class="datagridstyle" cellspacing="0" cellpadding="3" border="0" id="Datagrid3" width="100%">
	<tr class="datagridhead">
		<td>下载标题</td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学本科课程层次关系一览表-201809.xlsx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学本科课程层次关系一览表-201809.xlsx</a></td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学本科生“外语类”课程修读管理办法（2018 年4 月修订）.pdf','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学本科生“外语类”课程修读管理办法（2018 年4 月修订）.pdf</a></td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/关于公布《微积分（甲）Ⅰ》等数学物理化学类课程层次关系的通知-201611.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">关于公布《微积分（甲）Ⅰ》等数学物理化学类课程层次关系的通知-201611.docx</a></td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/新平台选课操作手册（2016年6月）.docx','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">新平台选课操作手册（2016年6月）.docx</a></td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/本科课程选课流程及说明.doc','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">本科课程选课流程及说明.doc</a></td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学部分荣誉课程、竺院课程与大类课程层次关系.xls','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学部分荣誉课程、竺院课程与大类课程层次关系.xls</a></td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学本科教学大类课程层次关系一览表-2014.xls','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学本科教学大类课程层次关系一览表-2014.xls</a></td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学2013-2014学年校历.xls','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学2013-2014学年校历.xls</a></td>
	</tr><tr>
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学本科生免听申请表.doc','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学本科生免听申请表.doc</a></td>
	</tr><tr class="datagrid1212">
		<td align="Left"><a href="#" onclick="window.open('http://jwbinfosys.zju.edu.cn/wbwj/浙江大学学生降级审批单.doc','gxlb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1,width=600,height=400,left=120,top=60')">浙江大学学生降级审批单.doc</a></td>
	</tr>
</table>
							</div>
						</div>
						<div class="de_gs">
							<h3><span>成绩更正公示</span><a href="cjgs.aspx" target="_blank"></a></h3>
							<div class="con">

								<marquee onmouseover="this.stop()" onmouseout="this.start()" scrollAmount="1" scrollDelay="70" direction="up" >
								<table class="datagridstyle" cellspacing="0" cellpadding="3" border="0" id="Datagrid4" width="100%">
	<tr>
		<td align="Left"></td>
	</tr><tr class="datagrid1212">
		<td align="Left"></td>
	</tr><tr>
		<td align="Left"></td>
	</tr>
</table>
								</marquee>
							</div>
						</div>
						<!--下载-->
						<div class="de_search">
							<h3><span>文件通知搜索</span></h3>
							<div class="con">
								<input name="Text1" id="Text1" type="text" size="18" />
								<button language="javascript" onclick="__doPostBack('BUTTON3','')" id="BUTTON3" class="de_btn_search" type="button">搜 索</button>
							</div>
						</div>

					</div>
				</div>
				<table>
					<tr>
						<td>
							<a href="http://zupo.zju.edu.cn" target="_blank"><IMG src="images/zupologo.gif" border="0"></a>
						</td>
						<td>
							<a href="http://jwb.zju.edu.cn" target="_blank"><IMG src="images/jwblogo.gif" border="0"></a></td>
						<td>
							<a href="http://10.10.10.151/" target="_blank"><IMG src="images/kczxlogo.gif" border="0">
							</a>
						</td>
						<td><a href="http://www.cc98.org/" target="_blank"><IMG src="images/cc98logo.gif" border="0">
							</a>
						</td>
					</tr>
				</table>
			</div>
			<div class="indexbot">版权所有&copy 浙江大学本科生院</div>



		</form>
	</body>
</HTML>
    "##;
