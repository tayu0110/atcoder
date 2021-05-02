#define UNICODE
#include<windows.h>
#include<windowsx.h>
#include<tchar.h>
#include"resourse.h"

inline int TestMessage(HWND hWnd){return MessageBox(hWnd, TEXT("TEST"), TEXT("TEST"), MB_OK);}
inline INT AboutMessage(HWND hWnd, TCHAR *szAbout){return ShellAbout(hWnd, TEXT("ABOUT"), szAbout, NULL);}
inline HFONT MyCreateFont(int nHeight, DWORD dwCharSet, LPCTSTR lpName){return CreateFont(nHeight, 0, 0, 0, FW_DONTCARE, FALSE, FALSE, FALSE, dwCharSet, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, DEFAULT_PITCH | FF_DONTCARE, lpName);}

const LPCTSTR FONT_MSMINCHO = TEXT("ＭＳ　明朝");
const LPCTSTR FONT_MSPMINCHO = TEXT("ＭＳ　Ｐ明朝");
const LPCTSTR FONT_HGGYOUSHOTAI = TEXT("HG行書体");
const LPCTSTR FONT_MSGOTHIC = TEXT("ＭＳ　ゴシック");
const LPCTSTR FONT_MSPGOTHC = TEXT("ＭＳ　Ｐゴシック");
const LPCTSTR FONT_MSUICOTHIC = TEXT("MS UI Gothic");
const LPCTSTR FONT_YUMINCHO = TEXT("游明朝");
const LPCTSTR FONT_YUGOTHIC = TEXT("游ゴシック");

LRESULT CALLBACK WndProc(HWND, UINT, WPARAM,LPARAM);
ATOM InitApp(HINSTANCE);
BOOL InitInstance(HINSTANCE, int);

TCHAR szClassName[] = TEXT("winapp");
TCHAR szAbout[] = TEXT("Copyright (C) 2020\n\tY.Tatsumi\n\t\n");

int WINAPI WinMain(HINSTANCE hCurInst, HINSTANCE hPrevInst, LPSTR lpsCmdLine, int nCmdShow){
    MSG msg;
    BOOL bRet;

    if(!InitApp(hCurInst)) return FALSE;
    if(!InitInstance(hCurInst, nCmdShow)) return FALSE;
    while((bRet =GetMessage(&msg, NULL, 0, 0)) != 0){
        if(bRet == -1){
            MessageBox(NULL, TEXT("GetMessage Error"), TEXT("Error"), MB_OK);
            break;
        }else{
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
    }
    return (int)msg.wParam;
}

ATOM InitApp(HINSTANCE hInst){
    WNDCLASSEX wc;
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WndProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hInstance = hInst;
    wc.hIcon = (HICON)LoadImage(NULL, IDI_APPLICATION, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED);
    wc.hCursor = (HCURSOR)LoadImage(NULL, IDC_ARROW, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED);
    wc.hbrBackground = (HBRUSH)GetStockObject(WHITE_BRUSH);
    wc.lpszMenuName = TEXT("MYMENU");
    wc.lpszClassName = szClassName;
    wc.hIconSm = (HICON)LoadImage(NULL, IDI_APPLICATION, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED);

    return (RegisterClassEx(&wc));
}

BOOL InitInstance(HINSTANCE hInst, int nCmdShow){
    HWND hWnd;

    hWnd = CreateWindow(szClassName, TEXT("GCCでウィンドウを作れるかの実験"),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
        NULL, NULL, hInst, NULL);
    if(!hWnd) return FALSE;

    ShowWindow(hWnd, nCmdShow);
    UpdateWindow(hWnd);
    return TRUE;
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT msg, WPARAM wp, LPARAM lp){
    int id;

    switch(msg){
        // case WM_CREATE:
        //     break;
        // case WM_TIMER:
        //     break;
        // case WM_SIZE:
        //     break;
        // case WM_SETFOCUS:
        //     break;
        // case WM_PAINT:
        //     break;
        // case WM_KEYDOWN:
        //     break;
        // case WM_KEYUP:
        //     break;
        // case WM_CHAR:
        //     break;
        // case WM_LBUTTONUP:
        //     break;
        // case WM_LBUTTONDOWN:
        //     break;
        // case WM_LBUTTONDBLCLK:
        //     break;
        // case WM_RBUTTONUP:
        //     break;
        // case WM_RBUTTONDOWN:
        //     break;
        // case WM_RBUTTONDBLCLK:
        //     break;
        // case WM_MBUTTONUP:
        //     break;
        // case WM_MBUTTONDOWN:
        //     break;
        // case WM_MBUTTONDBLCLK:
        //     break;
        // case WM_MOUSEMOVE:
        //     break;
        // case WM_COPY:
        //     break;
        // case WM_CUT:
        //     break;
        // case WM_PASTE:
        //     break;
        case WM_COMMAND:
            switch(LOWORD(wp)){
                case IDM_END:
                    SendMessage(hWnd, WM_CLOSE, 0, 0);
                    break;
            }
            break;
        case WM_CLOSE:
            id = MessageBox(hWnd, TEXT("May I terminate?"), TEXT("CAUTION"), MB_YESNO);
            if(id == IDYES){
                DestroyWindow(hWnd);
            }
            break;
        case WM_DESTROY:
            PostQuitMessage(0);
            break;
        default:
            return (DefWindowProc(hWnd, msg, wp, lp));
    }
    return 0;
}
