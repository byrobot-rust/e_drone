// 명령어 수신 상태 (Serial 클래스를 상속받는 클래스에서 사용함)
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum State {
    Ready,     // 명령어 수신 대기
    Receiving, // 명령어 수신중
    Loaded,    // 명령어 수신 완료 후 명령어 보관소에 대기중

    Failure, // 명령어 수신 실패
}


// 데이터 섹션 구분
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Section {
    Start,  // 전송 시작 코드
    Header, // 헤더
    Data,   // 데이터
    End,    // 데이터 확인
}
