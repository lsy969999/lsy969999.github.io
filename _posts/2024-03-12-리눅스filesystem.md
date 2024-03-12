---
layout: post
title:  "리눅스filesystem"
date:   2024-03-12 05:04:00 +0900
categories: linux
---
FHF(Filesystem Hierarchy Standard) 표준 파일 시스템
```
/: 루트 디렉토리
/bin: 기본적인 명령어
/boot: 부트로더 디렉토리
/dev: 시스템 디바이스(장치파일)
/etc: 각정 설정파일 
/home: 사용자의 홈 디렉토리
/lib: 라이브러리(및 커널모듈)
/media: 외부저장소(cdrom, usb 등)
/mnt: 외부 저장소 또는 파일시스템 마운트 포인트
/proc: 시스템 설정들의 가상파일시스템(pseudo-file시스템)
/root: 루트 사용자의 홈 디렉토리
/sbin: 시스템(관리자용) 명령어
/tmp: 임시저장소
/usr: 일반 사용자들 공통 파일
/var: 시스템 운용중에 생성되는 임시 데이터 저장소
```
/boot
grup 및 커널, initrd, 멀티 부팅 시 메모리 테스트를 위한 memtest도구

/home
사용자 디렉토리 및 슈퍼유저의 홈 디렉토리 /root

/etc
시스템 프로세서의 각종 설명 파일들
시스템 설정파일들이 여기에 모이게 된다.
```bash
# 운영체제 정보를 갖고 있는 파일들
cat /etc/*-release
```

/var 및 /var/log
시스템 프로세서의 각종 임시 파일들 및 로그 파일들

```bash
#파일시스템 명령어 - find
# find [options] [path] [expression]
find *.txt # 내 현재 디렉토리에서 확장자가 .txt인 파일을 찾는법
find . -name "*.txt" # 내 현재 디렉토리에서부터 확장자가 .txt인 파일 찾는법
find . -name "hello*" -type f # 내 현재 디렉토리에서부터 확장자가 hello로 시작하는 "파일"만 찾는 법
find . -name "dir*" -type d # 내 현재 디렉토리에서부터 dir로 시작하는 "디렉토리"만 찾는 법

find . -size +100000000c # 내 현재 디렉토리에서 용량이 100,000,000바이트 보다 큰 파일 찾는법
find -newerct "15 May 2020" -ls # 최근 생성된 파일만 찾아보기(2020.5.15 이후)
find . -mtine +2 -a -mtime -5 ls # 최근 2일에서 5일 사이에 변경된 파일 찾기
```

```bash
# 파일시스템 명령어 - 속성(stat)
# 원하는 파일의 속성(주로시간) 확인
# 시간의 유형
# Access: 파일에 최근 접근 시간(고전적으로는 read시에도 올라갔으나 지금은 다소 변경 됨)
# Modify: 파일의 내용 변경 시간
# Change: 파일의 수정 시간(inode 관점에서의 변화 시간 - 생성, 변경, 속성수정 등)
```

```bash
# 검색 명령어 - 필터링(grep)
# grep [options] pattern [file]
grep "usage" [file]
grep "vim" [file] # 대소문자 구분
grep -i "Vim" [file] # 대소문자 무시
grep -r "vim" [path] # 하위 디렉토리 모두 검색

```

```bash
# 소팅
# sort [option] [file]
# 파일의 내용을 특정 순서(옵션)로 정렬(파이프와 많이 연동해서 사용)
ls -l | sort # 디렉토리  목록을 소팅(기본값: 첫번째 컬럼)

```

```bash
# 내용 검색/편집 명령어(awk)
# 패턴 검색 및 텍스트 프로세싱
ls -l | awk '{print $1}'# 디렉토리 목록 중 첫번째 컬럼만 출력
ls -l | awk '{print "FILENAME:"$9, "SIZE:"$5}' # 디렉토리 목록 중 파일명과 사이즈만 출력
```

```bash
# 내용 검색/편집 명령어(sed)
# 's/패턴/변환/g'
# 스트림 라인 편집기(search and replace)
cat /usr/share/doc/vim/copyright | sed 's/book/books/g' # 파일내의 모든 book을 books 로 변환
```

```bash
# 중복제거 유틸(uniq) 및 단어 분석(word-count)
# 파일 내에 중복되는 줄 제거
cat hello.txt | uniq # 파일 내에 중복되는 줄 제거
wc hello.txt # 파일내의 "라인수/단어수/문자수" 출력
wc -l hello.txt # 라인수만 출력

```

```bash
# 디스크 용량 -du(disk usage)
# du [option] [file]
du -k # 결과값 kb 단위 출력(기본값)
du -m # 결과값 mb 단위 출력
du -h # 사용자 편의용량(kb/mb/gb, 1m=2^20)
du -H # 사용자 편의용량(kb/mb/gb, 1m=1,000,000)
du -s # 합계만 출력
du -S # 서브 디렉토리 용량 함지지 않고, 각각 계산

du -S | sort -n # 디렉토리별 용량을 오름차순 소팅해서 출력
du / -h 2> /dev/null | grep [0-9]G # 디렉토리별 누적 용량을 출력하여 GB 이상의 디렉토리 출력)
du --max-depth=1 # 디렉토리 용량을 최대 1디렉토리 depth 까지만 출력

df
df -h
df -T
```

```bash
# 묶음/압축 tar(tape archive)
# tar [option] [file] [path]
# c: create(생성)
# x: extract(해지)
# v: verbose(디테일한 상황 보고)
# f: file(저장될 파일명 지정하기 위해)
# t: list(목록 확인)
# z: zip(압축)
# j: bzip2(압축)
tar cvf myzip.tar dir1 # tar 아카이브 만들기
tar cvfz myzip.tgz # 압축
tar xvf myzip.tar # 풀기
tar xvfz myzip.tgz # 압축 풀기
```

```bash
#압축 gz, bz2, xz
# gzip [option] [file]

```