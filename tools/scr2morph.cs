using System;
using System.Collections.Generic;
using System.Text;

public class LgScrErrorV2 : Exception { public LgScrErrorV2(string m) : base(m) {} }

public class Scr2MorphV2
{
    class Tok { public int K; public string S; public int L; public Tok(int k, string s, int l) { K = k; S = s; L = l; } }
    const int ID = 0, NUM = 1, STR = 2, VEC = 3, DOL = 4, OP = 5, EOF = 6;
    static readonly HashSet<string> Types = new HashSet<string> { "void", "float", "int", "string", "entity", "vector" };
    static readonly string[] Ops2 = { "==", "!=", "<=", ">=", "&&", "||", "++", "--", "+=", "-=", "*=", "/=" };
    static readonly string[][] Lv = { new[] { "||" }, new[] { "&&" }, new[] { "|" }, new[] { "^" }, new[] { "&" },
        new[] { "==", "!=" }, new[] { "<", ">", "<=", ">=" }, new[] { "+", "-" }, new[] { "*", "/", "%" } };

    public static Dictionary<string, int> UnkFn = new Dictionary<string, int>();
    public static Dictionary<string, int> UnkMe = new Dictionary<string, int>();
    public static List<string> Warn = new List<string>();
    public static void Reset() { UnkFn.Clear(); UnkMe.Clear(); Warn.Clear(); }

    static HashSet<string> NewSet() { return new HashSet<string>(StringComparer.OrdinalIgnoreCase); }
    List<Tok> T; int p; string Root; public string Src;
    HashSet<string> Funcs = NewSet(), Globals = NewSet(), Locals = NewSet();
    List<string> Pre = new List<string>(); int Tmp; StringBuilder O = new StringBuilder(); int Ind;

    public static string Resolve(string path, Dictionary<string, string> files, HashSet<string> seen, List<string> warn)
    {
        string key = path.ToLowerInvariant().Replace('\\', '/');
        if (seen.Contains(key)) return "";
        seen.Add(key);
        string txt;
        if (!files.TryGetValue(key, out txt)) { warn.Add("include ausente: " + path); return ""; }
        var sb = new StringBuilder();
        foreach (var raw in txt.Split('\n'))
        {
            string ln = raw.TrimEnd('\r'); string t = ln.Trim();
            if (t.StartsWith("#include"))
            {
                int a = t.IndexOf('"'), b = t.LastIndexOf('"');
                if (a >= 0 && b > a) { sb.Append(Resolve(t.Substring(a + 1, b - a - 1), files, seen, warn)); sb.Append('\n'); continue; }
            }
            sb.Append(ln); sb.Append('\n');
        }
        return sb.ToString();
    }

    static List<Tok> Lex(string s)
    {
        var r = new List<Tok>(); int i = 0, n = s.Length, line = 1;
        while (i < n)
        {
            char c = s[i];
            if (c == '\n') { line++; i++; continue; }
            if (char.IsWhiteSpace(c)) { i++; continue; }
            if (c == '/' && i + 1 < n && s[i + 1] == '/') { while (i < n && s[i] != '\n') i++; continue; }
            if (c == '/' && i + 1 < n && s[i + 1] == '*') { i += 2; while (i + 1 < n && !(s[i] == '*' && s[i + 1] == '/')) { if (s[i] == '\n') line++; i++; } i += 2; continue; }
            if (c == '#') { while (i < n && s[i] != '\n') i++; continue; }
            if (c == '"')
            {
                int j = i + 1; var sb = new StringBuilder();
                while (j < n && s[j] != '"') { if (s[j] == '\\' && j + 1 < n) { sb.Append(s[j]); j++; } if (s[j] == '\n') line++; sb.Append(s[j]); j++; }
                r.Add(new Tok(STR, sb.ToString(), line)); i = j + 1; continue;
            }
            if (c == '\'') { int j = s.IndexOf('\'', i + 1); if (j < 0) j = n; r.Add(new Tok(VEC, s.Substring(i + 1, j - i - 1), line)); i = j + 1; continue; }
            if (char.IsDigit(c) || (c == '.' && i + 1 < n && char.IsDigit(s[i + 1])))
            { int j = i; while (j < n && (char.IsDigit(s[j]) || s[j] == '.')) j++; r.Add(new Tok(NUM, s.Substring(i, j - i), line)); i = j; continue; }
            if (c == '$' || char.IsLetter(c) || c == '_')
            {
                int j = i + 1; while (j < n && (char.IsLetterOrDigit(s[j]) || s[j] == '_')) j++;
                if (c == '$') r.Add(new Tok(DOL, s.Substring(i + 1, j - i - 1), line)); else r.Add(new Tok(ID, s.Substring(i, j - i), line));
                i = j; continue;
            }
            string op = c.ToString();
            if (i + 1 < n) { string two = s.Substring(i, 2); if (Array.IndexOf(Ops2, two) >= 0) op = two; }
            r.Add(new Tok(OP, op, line)); i += op.Length;
        }
        r.Add(new Tok(EOF, "<EOF>", line)); return r;
    }

    Tok At(int k) { return T[Math.Min(k, T.Count - 1)]; }
    Tok Cur { get { return At(p); } }
    static bool IsK(Tok t, string s) { return (t.K == OP || t.K == ID) && t.S == s; }
    bool Is(string s) { return IsK(Cur, s); }
    void Exp(string s) { if (!Is(s)) Fail("esperado '" + s + "'"); p++; }
    string Id() { if (Cur.K != ID) Fail("esperado identificador"); return T[p++].S; }
    void Fail(string m) { string[] ls = (Src ?? "").Split('\n'); int k = Cur.L - 1; string ctx = (k >= 0 && k < ls.Length) ? ls[k].Trim() : ""; throw new LgScrErrorV2(Root + " linha(expandida) " + Cur.L + ": " + m + " (token '" + Cur.S + "') >> " + ctx); }
    static void Count(Dictionary<string, int> d, string k) { int v; d.TryGetValue(k, out v); d[k] = v + 1; }
    void Line(string s) { O.Append(new string('\t', Ind)); O.Append(s); O.Append('\n'); }
    void FlushPre() { foreach (var x in Pre) Line(x); Pre.Clear(); }
    string Hoist(string s) { string tv = "local.lg_t" + (++Tmp); Pre.Add(tv + " = " + s); return tv; }
    static string Default(string ty) { switch (ty) { case "float": case "int": return "0"; case "string": return "\"\""; case "vector": return "( 0 0 0 )"; default: return null; } }
    static string Num(string s) { if (s.StartsWith(".")) s = "0" + s; if (s.EndsWith(".")) s = s + "0"; return s; }
    static string Vec(string s)
    {
        var sb = new StringBuilder("(");
        foreach (var x in s.Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries)) { sb.Append(' '); sb.Append(Num(x)); }
        sb.Append(" )"); return sb.ToString();
    }
    static string Arg(string x) { if (x.IndexOf(' ') < 0) return x; if (x.StartsWith("(") || x.StartsWith("\"")) return x; return "(" + x + ")"; }
    static string Join(List<string> a) { var sb = new StringBuilder(); foreach (var x in a) { sb.Append(' '); sb.Append(Arg(x)); } return sb.ToString(); }

    string Var(string n)
    {
        if (Locals.Contains(n)) return "local." + n;
        if (Globals.Contains(n)) return "level." + n;
        Warn.Add(Root + ": variavel nao declarada " + n); Locals.Add(n); return "local." + n;
    }

    public static string Translate(string root, Dictionary<string, string> files)
    {
        var w = new List<string>();
        string src = Resolve(root, files, NewSet(), w);
        foreach (var x in w) Warn.Add(root + ": " + x);
        var s = new Scr2MorphV2(); s.Root = root; s.Src = src; s.T = Lex(src);
        return s.Run();
    }

    string Run()
    {
        int depth = 0, pd = 0;
        for (int i = 0; i < T.Count; i++)
        {
            var t = T[i];
            if (IsK(t, "{")) depth++; else if (IsK(t, "}")) depth--;
            else if (IsK(t, "(")) pd++; else if (IsK(t, ")")) pd--;
            else if (depth == 0 && pd == 0 && t.K == ID && Types.Contains(t.S) && At(i + 1).K == ID)
            {
                if (IsK(At(i + 2), "(")) Funcs.Add(At(i + 1).S);
                else
                {
                    Globals.Add(At(i + 1).S);
                    for (int j = i + 2; j < T.Count && !IsK(T[j], ";"); j++) if (IsK(T[j], ",") && At(j + 1).K == ID) Globals.Add(At(j + 1).S);
                }
            }
        }
        var globInit = new List<string>(); var fns = new StringBuilder(); var done = NewSet();
        while (Cur.K != EOF)
        {
            if (Is(";")) { p++; continue; }
            if (Cur.K != ID || !Types.Contains(Cur.S)) Fail("esperado declaracao no topo");
            string ty = T[p++].S; string name = Id();
            if (Is("("))
            {
                p++; var pars = new List<string>(); Locals = NewSet();
                if (Is("void") && IsK(At(p + 1), ")")) p++;
                while (!Is(")"))
                {
                    if (Cur.K == ID && Types.Contains(Cur.S)) p++;
                    string pn = Id(); pars.Add(pn); Locals.Add(pn);
                    if (Is(",")) p++; else if (!Is(")")) Fail("parametro");
                }
                p++;
                if (Is(";")) { p++; continue; } O = new StringBuilder(); Ind = 1; Tmp = 0; Pre = new List<string>();
                Block();
                if (done.Contains(name)) { Warn.Add(Root + ": funcao duplicada " + name + " (mantida a 1a)"); continue; }
                done.Add(name);
                fns.Append("\n" + name); foreach (var a in pars) fns.Append(" local." + a); fns.Append(":\n");
                fns.Append(O.ToString()); fns.Append("end\n");
            }
            else
            {
                Locals = NewSet(); Pre = new List<string>();
                while (true)
                {
                    string val = Default(ty);
                    if (Is("=")) { p++; val = Expr(); if (Pre.Count > 0) Fail("init global com chamada"); }
                    if (val != null) globInit.Add("level." + name + " = " + val);
                    if (Is(",")) { p++; name = Id(); continue; }
                    break;
                }
                Exp(";");
            }
        }
        var o = new StringBuilder();
        o.Append("// LG scr2morph v2: " + Root + "\n");
        foreach (var g in globInit) o.Append(g + "\n");
        if (Funcs.Contains("main")) o.Append("thread main\n");
        o.Append("end\n");
        o.Append(fns.ToString());
        return o.ToString();
    }

    void Block() { Exp("{"); while (!Is("}")) { if (Cur.K == EOF) Fail("EOF dentro de bloco"); Stmt(); } p++; }
    void Body() { Line("{"); Ind++; if (Is("{")) Block(); else Stmt(); Ind--; Line("}"); }

    void Stmt()
    {
        Pre = new List<string>();
        var t = Cur;
        if (Is(";")) { p++; return; }
        if (Is("{")) { Block(); return; }
        if (t.K == ID && Types.Contains(t.S) && At(p + 1).K == ID)
        {
            string ty = T[p++].S;
            while (true)
            {
                string n = Id(); Locals.Add(n);
                string v = Default(ty);
                if (Is("=")) { p++; v = Expr(); }
                FlushPre();
                if (v != null) Line("local." + n + " = " + v);
                if (Is(",")) { p++; continue; }
                break;
            }
            Exp(";"); return;
        }
        if (Is("thread"))
        {
            p++; string f = Id(); Exp("("); var a = ArgList(); Exp(";");
            if (!Funcs.Contains(f)) Warn.Add(Root + ": thread de funcao inexistente " + f);
            FlushPre(); Line("thread " + f + Join(a)); return;
        }
        if (Is("if"))
        {
            p++; Exp("("); string c = Expr(); Exp(")"); FlushPre();
            Line("if (" + c + ")"); Body();
            if (Is("else")) { p++; Line("else"); Body(); }
            return;
        }
        if (Is("while"))
        {
            p++; Exp("("); string c = Expr(); Exp(")");
            if (Pre.Count == 0) { Line("while (" + c + ")"); Body(); return; }
            var pre = new List<string>(Pre); Pre.Clear();
            Line("while (1)"); Line("{"); Ind++;
            foreach (var x in pre) Line(x);
            Line("if (!(" + c + "))"); Line("{"); Ind++; Line("break"); Ind--; Line("}");
            if (Is("{")) Block(); else Stmt();
            Ind--; Line("}"); return;
        }
        if (Is("for"))
        {
            p++; Exp("(");
            string ini = Is(";") ? "" : Simple(); Exp(";");
            FlushPre();
            string c = Is(";") ? "1" : Expr(); Exp(";");
            string inc = Is(")") ? "" : Simple(); Exp(")");
            if (Pre.Count > 0) Fail("chamada na condicao/incremento do for");
            Line("for (" + ini + " ; " + c + " ; " + inc + ")"); Body(); return;
        }
        if (Is("return"))
        {
            p++; if (Is(";")) { p++; Line("end"); return; }
            string v = Expr(); Exp(";"); FlushPre(); Line("end " + Arg(v)); return;
        }
        if (Is("break") || Is("continue")) { string k = T[p++].S; Exp(";"); Line(k); return; }
        if (Is("else")) Fail("else solto");
        string s = Simple(); Exp(";"); FlushPre(); if (s.Length > 0) Line(s);
    }

    string Simple()
    {
        if (Cur.K == ID && !Funcs.Contains(Cur.S) && !Types.Contains(Cur.S))
        {
            Tok nx = At(p + 1);
            if (nx.K == OP && (nx.S == "=" || nx.S == "+=" || nx.S == "-=" || nx.S == "*=" || nx.S == "/="))
            {
                string lv = Var(T[p].S); p += 2; string v = Expr();
                if (nx.S == "=") return lv + " = " + v;
                return lv + " = (" + lv + " " + nx.S.Substring(0, 1) + " " + v + ")";
            }
            if (nx.K == OP && (nx.S == "++" || nx.S == "--")) { string lv = Var(T[p].S); p += 2; return lv + nx.S; }
        }
        if (Is("++") || Is("--")) { string op = T[p++].S; return Var(Id()) + op; }
        string e = Expr();
        if (Pre.Count > 0 && e.StartsWith("local.lg_t") && Pre[Pre.Count - 1].StartsWith(e + " = "))
        { string c = Pre[Pre.Count - 1].Substring(e.Length + 3); Pre.RemoveAt(Pre.Count - 1); return c; }
        Warn.Add(Root + ": expressao sem efeito linha(expandida) " + Cur.L); return "";
    }

    string Expr() { return Bin(0); }
    string Bin(int l)
    {
        if (l == Lv.Length) return Unary();
        string a = Bin(l + 1);
        while (Cur.K == OP && Array.IndexOf(Lv[l], Cur.S) >= 0) { string op = T[p++].S; string b = Bin(l + 1); a = "(" + a + " " + op + " " + b + ")"; }
        return a;
    }
    string Unary()
    {
        if (Is("!")) { p++; return "(!" + Unary() + ")"; }
        if (Is("-")) { p++; if (Cur.K == NUM) return "-" + Num(T[p++].S); return "(-" + Unary() + ")"; }
        if (Is("+")) { p++; return Unary(); }
        return Postfix();
    }
    List<string> ArgList()
    {
        var r = new List<string>();
        while (!Is(")")) { if (Cur.K == EOF) Fail("EOF em argumentos"); r.Add(Expr()); if (Is(",")) p++; else if (!Is(")")) Fail("argumento"); }
        p++; return r;
    }
    string Call(string n)
    {
        p++;
        var a = ArgList();
        if (n == "getentity" && a.Count == 1) return "$(" + a[0] + ")";
        if (Funcs.Contains(n)) return Hoist("waitthread " + n + Join(a));
        Count(UnkFn, n);
        if (n == "trigger")
            for (int i = 0; i < a.Count; i++)
            { string x = a[i]; if (x.Length > 3 && x[0] == '"' && x[1] == '$' && x[x.Length - 1] == '"' && x.IndexOf(' ') < 0) a[i] = "$" + x.Substring(2, x.Length - 3); }
        return Hoist(n + Join(a));
    }
    string Postfix()
    {
        string obj = null; var t = Cur;
        if (t.K == NUM) { p++; return Num(t.S); }
        if (t.K == STR) { p++; return "\"" + t.S + "\""; }
        if (t.K == VEC) { p++; return Vec(t.S); }
        if (Is("(")) { p++; string e = Expr(); Exp(")"); obj = e; }
        else if (t.K == DOL)
        {
            p++;
            if (t.S.Length == 0 && Is("(")) { p++; string e = Expr(); Exp(")"); obj = "$(" + e + ")"; }
            else obj = "$" + t.S;
        }
        else if (t.K == ID)
        {
            p++; string n = t.S;
            if (n == "true") return "1";
            if (n == "false") return "0";
            if (n == "NULL") return "NIL";
            if (Is("(")) obj = Call(n);
            else if (Locals.Contains(n) || Globals.Contains(n)) obj = Var(n);
            else if (Is(".")) obj = "$" + n;
            else { Warn.Add(Root + ": identificador solto " + n); obj = "$" + n; }
        }
        else Fail("expressao");
        while (Is("."))
        {
            p++; string m = Id(); Exp("("); var a = ArgList();
            Count(UnkMe, m);
            obj = Hoist(Arg(obj) + " " + m + Join(a));
        }
        return obj;
    }
}