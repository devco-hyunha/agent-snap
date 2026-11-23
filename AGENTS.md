# snap

로컬 LLM용 독립 코딩 에이전트입니다.

<!-- agent-harness:begin -->
## 기획·개발 하네스 핵심 규칙

정본은 harness/HARNESS.md입니다. 스킬: harness-router, plan-harness, project-plan-harness, build-harness, review-harness, delegation-brief, proofread-harness, eli5-clarity, dev-principles. 분기: harness-router → playbooks 경로만. 절차 장문은 플레이북·HARNESS(3·6·7절). 서브 위임 시 자기점검에 하네스 준수 칸 필수(delegation-brief).

### 공통 핵심 (전원 필수, HARNESS.md 1절)
- 요청이 오면 harness-router로 플레이북 하나를 고르고 그 경로만 읽습니다. 동시 둘 이상 서브 금지·순차 허용.
- 판단 정본(하네스·오버레이·기획/작업 문서·해당 플레이북/SKILL)은 메인이 직접 읽고, 잡음 절약은 대량 검색·긴 로그·리서치에만 둡니다.
- 사용자 언어·eli5-clarity를 씁니다. 존댓말=응답·서브 지시·보고·문서·커밋. 코드 주석·독스트링=쉬운 명사형 한 줄(eli5 정본).
- 한자·외국어 서술과 깨진 글자(U+FFFD)를 쓰지 않습니다.
- 코드·경로·명령·식별자·인용은 원문 그대로 둡니다.
- 마커 블록과 harness/HARNESS.md는 읽기 전용이라 고치지 않습니다.
- 프로젝트 규칙은 마커 바깥 `## 프로젝트 오버레이`(또는 docs/PROJECT_RULES.md)에 둡니다.
- 더 엄격한 오버레이가 우선하고, 완화는 `[안전 예외 승인: N번]`을 명시한 때만 허용합니다.
- harness/VERSION과 .gitattributes(`eol=lf`)를 함께 둡니다.

### 승인·안전 6원칙 (전원 필수, HARNESS.md 5절)
1. 기획 승인 전 코드를 쓰지 않습니다.
2. 구현 중 멈춤은 새 권한·비가역(파일 삭제 포함)·보안·제품 결과 변경·동일 지적 보정 2회 후 미달뿐입니다. 파일 삭제 요청·수행·보고 시 해당 파일이 무엇인지, 삭제 이유와 근거가 무엇인지 반드시 설명합니다.
3. 커밋·푸시·병합은 사용자 허락이 필요합니다(오버레이 명시 예외 제외). 메시지 형식·절차는 skills/dev-principles/leaves/commit.md를 따릅니다.
4. 비밀 값·.env를 커밋·로그에 남기지 않습니다.
5. 리뷰 없이 PASS하지 않습니다. 표준은 독립 리뷰 세션, Fast-track은 메인 셀프 체크가 판정합니다.
6. 요청받지 않은 코드·미지시 파일·임시 검증 스크립트(_verify*.py)를 만들지 않습니다. 검증은 -c 인라인만 씁니다.
<!-- agent-harness:end -->

## 프로젝트 오버레이

[안전 예외 승인: 3번 커밋 — 작업 단위(= `docs/plans/`의 세부 작업 하나)의 전체 리뷰 PASS 기록이 있고, 그 결과를 사용자에게 보고한 뒤 로컬 커밋을 한다(푸시·병합은 사용자 허락 필수)]

- 오버레이 본문: `harness/PROJECT_RULES.md` (하네스 정본은 `harness/HARNESS.md`).
- 하네스 원본: `https://github.com/devco-hyunha/agent-harness`(public 저장소)를 원본으로 쓴다.
