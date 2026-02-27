class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.46"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.46"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.46/six7-0.7.46-arm64-apple-darwin.tar.gz"
      sha256 "59df72efe5e7222dbcd0b4f5f8edef623ee8f50fc5653b0056812de953d14137"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
