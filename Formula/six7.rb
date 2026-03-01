class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.47"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.47"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.47/six7-0.7.47-arm64-apple-darwin.tar.gz"
      sha256 "0482e15ccc543307bddbcad73cd9d81480f0c0a269323bcb5e6cf6daf05d9801"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
